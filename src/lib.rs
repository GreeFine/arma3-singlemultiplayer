#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![feature(rustc_private)]
#![feature(libc)]
extern crate libc;

use libc::{size_t, strncpy};
use std::{error::Error, ffi::CString};
// use std::net::UdpSocket;
use std::os::raw::*;

mod commands;
mod convert;

unsafe fn craft_responsse(responsse: String, dest: *mut i8, dest_size: c_int) {
    let response_buff = CString::new(responsse).unwrap().into_raw();
    strncpy(dest, response_buff, (dest_size - 1) as usize);
}

fn error_exit(error: Box<dyn Error>, a3_output: *mut i8, a3_output_size: c_int) -> c_int {
    let responsse = format!("Errored: {}", error);
    unsafe {
        craft_responsse(responsse, a3_output, a3_output_size);
    }
    1
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
    a3_output_size: c_int,
    a3_function: *const c_char,
    a3_argv: *const *const c_char,
    a3_argc: c_int,
) -> c_int {
    let function_str_res = convert::arma_str(a3_function);
    let function_str;
    if let Err(error) = function_str_res {
        return error_exit(error, a3_output, a3_output_size);
    } else {
        function_str = function_str_res.unwrap();
    }

    let mut arguments: Vec<_> = Vec::new();
    for index in 0..a3_argc {
        let argument_res = convert::arma_str(*a3_argv.offset(index as isize));
        let argument;
        if let Err(error) = argument_res {
            return error_exit(error, a3_output, a3_output_size);
        } else {
            argument = argument_res.unwrap();
        }
        println!("index: {}, arg: {}", index, argument);
        arguments.push(argument);
    }

    let result = match &*function_str {
        "fetch" => commands::fetch(&arguments),
        "send" => commands::send(&arguments),
        "test" => Ok(arguments.join(" | ")),
        _ => Ok(String::from("command not found")),
    };
    let responsse = result.unwrap_or_else(|err| format!("Errored: {}", err));
    craft_responsse(responsse, a3_output, a3_output_size);

    a3_argc as c_int
}

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
