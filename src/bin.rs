use std::env;
use std::ffi::{CStr, CString};
use std::str;
mod server;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        server::start().unwrap();
    } else {
        let function = &args[1];
        let fucntion_args = &args[2..];

        println!(
            "Testing lib with function: {} addr: {:#?}",
            &function, &fucntion_args
        );
        unsafe {
            rve_get_version();
            rve_get(function, fucntion_args);
        }
    }
}

unsafe fn rve_get_version() {
    let buffer = CString::new("____________________").unwrap().into_raw();

    let utf8_arr: &[u8] = CStr::from_ptr(buffer).to_bytes();
    println!("buffer: [{}]", str::from_utf8(utf8_arr).unwrap());
    lib::RVExtensionVersion(buffer, utf8_arr.len() + 1);
    println!("buffer result: [{}]", str::from_utf8(utf8_arr).unwrap());
}

unsafe fn rve_get(function: &str, function_args: &[String]) {
    let function_c = CString::new(function).unwrap().into_raw();
    let buffer = CString::new("________________________________________________________________")
        .unwrap()
        .into_raw();

    let utf8_arr: &[u8] = CStr::from_ptr(buffer).to_bytes();
    println!("buffer: [{}]", str::from_utf8(utf8_arr).unwrap());

    let mut args = Vec::new();
    for arg in function_args {
        let new_arg: *const i8 = CString::new(arg.clone()).unwrap().into_raw();
        args.push(new_arg);
    }

    let argv = args.as_ptr();
    lib::RVExtensionArgs(buffer, utf8_arr.len() + 1, function_c, argv, args.len());
    println!("buffer result: [{}]", str::from_utf8(utf8_arr).unwrap());
}
