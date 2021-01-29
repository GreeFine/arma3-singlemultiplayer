#![allow(unused_attributes)]
#![feature(rustc_private)]
#![feature(libc)]
extern crate libc;

use libc::{size_t, strncpy};
use std::ffi::CString;
// use std::net::UdpSocket;
use std::os::raw::*;

mod commands;
mod convert;

unsafe fn craft_responsse(responsse: String, dest: *mut i8, dest_size: size_t) {
    let response_buff = CString::new(responsse).unwrap().into_raw();
    strncpy(dest, response_buff, dest_size - 1);
}

/// # Safety
// Called by Arma: STRING callExtension STRING
// 	__declspec (dllexport) int __stdcall RVExtensionArgs(char *output, int outputSize, const char *function, const char **argv, int argc);
// This function link our DLL to arma, thus it cannot be Safe (raw pointers, etc...)
// https://community.bistudio.com/wiki/Extensions
#[allow(non_snake_case)]
#[no_mangle]
#[export_name = "RVExtensionArgs"]
pub unsafe extern "stdcall" fn RVExtensionArgs(
    a3_output: *mut c_char,
    a3_outputSize: size_t,
    a3_function: *const c_char,
    a3_argv: *const *const c_char,
    a3_argc: size_t,
) {
    let function_str = convert::arma_str(a3_function);
    let res = if a3_argc > 0 {
        let argument_0 = convert::arma_str(*a3_argv.offset(0));
        match &*function_str {
            "fetch" => commands::fetch(&argument_0),
            _ => String::from("command not found"),
        }
    } else {
        String::from("default command todo")
    };

    craft_responsse(res, a3_output, a3_outputSize);
}

/// # Safety
// Called by Engine on extension load
// __attribute__((dllexport)) void RVExtensionVersion(char *output, int outputSize);
// This function link our DLL to arma, thus it cannot be Safe (raw pointers, etc...)
// https://community.bistudio.com/wiki/Extensions
#[allow(non_snake_case)]
#[no_mangle]
#[export_name = "RVExtensionVersion"]
pub unsafe extern "stdcall" fn RVExtensionVersion(output: *mut c_char, outputSize: size_t) {
    let versionstr = "V0.1 DEBUG";
    let response = CString::new(versionstr).unwrap().into_raw();
    println!("size_t: {:#?}", outputSize);
    strncpy(output, response, outputSize - 1);
}
