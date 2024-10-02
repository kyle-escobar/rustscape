pub mod error;
pub use error::*;
pub use anyhow::*;

use std::ffi::c_void;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use windows::Win32::Foundation::{BOOL, HINSTANCE, TRUE};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

pub type Result<T, E = Error> = anyhow::Result<T, E>;

fn startup(module: HINSTANCE) -> Result<()> {
    unsafe { AllocConsole(); }
    SimpleLogger::new()
        .with_colors(true)
        .with_level(LevelFilter::Info)
        .init()?;
    log::info!("[+] Starting RustScape Core Internals...");

    Ok(())
}

fn shutdown() -> Result<()> {

    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    module: HINSTANCE,
    call_reason: u32,
    _: *mut c_void
) -> BOOL {
    match call_reason {
        1u32 => { startup(module); },
        0u32 => { shutdown(); },
        _ => ()
    }
    TRUE
}
