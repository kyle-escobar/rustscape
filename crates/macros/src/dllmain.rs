use proc_macro::TokenStream;
use std::cell::{LazyCell, OnceCell, RefCell, RefMut};
use syn::{parse_macro_input, ItemFn};
use quote::quote;
use windows::Win32::Foundation::HINSTANCE;



pub fn parse_dll_main_macro(
    args: TokenStream,
    input: TokenStream
) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    quote!(
        pub static mut BASE_MODULE: Option<HINSTANCE> = None;

        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn DllMain(
            hmodule: HINSTANCE,
            call_reason: u32,
            _: c_void
        ) -> bool {
            const DLL_PROCESS_ATTACH: u32 = 1u32;
            const DLL_PROCESS_DETACH: u32 = 0u32;

            crate::set_module(hmodule);

            match call_reason {
                DLL_PROCESS_ATTACH => { #block },
                DLL_PROCESS_DETACH => (),
                _ => ()
            };

            true
        }


        fn set_module(hmodule: HINSTANCE) {
            unsafe { BASE_MODULE = Some(hmodule) }
        }

        pub fn base_module() -> HINSTANCE {
            unsafe { BASE_MODULE.unwrap() }
        }

        pub fn base_address() -> usize {
            base_module().0 as usize
        }
    ).into()
}



