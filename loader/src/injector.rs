use obfstr::obfstr;
use std::path::Path;
use std::time::Duration;
use std::{io, mem, thread};
use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, GetLastError, FALSE, HANDLE, WAIT_FAILED};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_DECOMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::{CreateRemoteThread, OpenProcess, WaitForSingleObject, PROCESS_ALL_ACCESS};

pub fn inject_dll(
    process_name: &str,
    dll_path: &Path,
) -> Result<HANDLE, io::Error> {
    log::info!("Starting injection of RustScape DLL...");

    log::info!("Searching for running process with name: {}...", process_name);

    #[allow(unused_assignments)]
    let mut process_id = None;
    loop {
        process_id = get_pid_from_name(process_name);
        if process_id.is_none() {
            thread::sleep(Duration::from_millis(1));
            continue;
        } else {
            thread::sleep(Duration::from_millis(500));
            break;
        }
    }
    log::info!("Found process '{}'! [PID: {}]", process_name, process_id.unwrap());

    /*
     * Injection logic
     */

    log::info!("Injecting DLL into process...");

    let dll_path_str = dll_path.to_str().unwrap();
    let process_id = process_id.unwrap();
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id) }
        .map_err(|_| { panic!("Failed to open process [name={}, id={}] to get handle.", process_name, process_id); }).unwrap();

    let alloc_memory = unsafe {
        VirtualAllocEx(
            process_handle,
            None,
            dll_path_str.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if alloc_memory.is_null() {
        panic!("Failed to allocate memory in the process [name={}, id={}].", process_name, process_id);
    }

    let mut tmp = 0;
    if unsafe {
        WriteProcessMemory(
            process_handle,
            alloc_memory,
            dll_path_str.as_ptr() as _,
            dll_path_str.len(),
            Some(&mut tmp),
        )
    }.is_ok() == false {
        panic!("Failed to write to process memory.");
    }

    let kernel32_address = unsafe { GetModuleHandleA(PCSTR::from_raw(obfstr!("KERNEL32.DLL\0").as_ptr()))? };

    let load_library_address = unsafe {
        GetProcAddress(kernel32_address, PCSTR::from_raw(obfstr!("LoadLibraryA\0").as_ptr())).unwrap()
    };

    let mut tmp = 0;
    let thread_handle = unsafe {
        CreateRemoteThread(
            process_handle,
            None,
            0,
            Some(mem::transmute(load_library_address as usize)),
            Some(alloc_memory),
            0,
            Some(&mut tmp),
        )?
    };

    if unsafe { WaitForSingleObject(thread_handle, 100) } == WAIT_FAILED {
        unsafe { panic!("Failed to wait for the thread handle to release/exit it's context."); }
    }

    unsafe { VirtualFreeEx(
        process_handle,
        alloc_memory,
        1,
        MEM_DECOMMIT
    )? };

    unsafe { CloseHandle(thread_handle)? };
    unsafe { CloseHandle(process_handle)? };

    log::info!("Injection of DLL complete.");

    Ok(process_handle)
}

pub fn get_pid_from_name(process_name: &str) -> Option<u32> {
    let snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.unwrap();

    let mut entry: PROCESSENTRY32W = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    let mut found = false;

    if !unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
        panic!("Failed to call process32firstw().");
    }
    loop {
        let entry_name = String::from_utf16_lossy(
            &entry.szExeFile[..entry.szExeFile.iter().position(|v| *v == 0).unwrap_or(0)]
        );
        if entry_name == String::from(process_name) {
            found = true;
            break;
        }
        if unsafe { Process32NextW(snapshot, &mut entry) }.is_ok() == false {
            break;
        }
    }

    if found {
        Some(entry.th32ProcessID)
    } else {
        None
    }
}