#[macro_use]
extern crate proc_macro;
extern crate windows;

mod dllmain;
use dllmain::*;

use proc_macro::TokenStream;
pub(crate) use std::ffi::c_void;

#[proc_macro_attribute]
pub fn dll_main(args: TokenStream, input: TokenStream) -> TokenStream { dllmain::parse_dll_main_macro(args, input) }