use std::ffi::c_void;
use windows::Win32::Foundation::HINSTANCE;

#[rustscape_macros::dll_main]
fn main() {
    let x = 0;
    let y = 50;

    println!("Result: {}", x + y);

    let base_module = base_module();
    let base_address = base_address();

    println!("Base Addr: {:x}", base_address);
}