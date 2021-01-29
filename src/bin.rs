use std::ffi::{CStr, CString};
use std::str;

fn main() {
    unsafe {
        rve_get_version();
        rve_get();
    }
}

unsafe fn rve_get_version() {
    let buffer = CString::new("____________________").unwrap().into_raw();

    let utf8_arr: &[u8] = CStr::from_ptr(buffer).to_bytes();
    println!("buffer: [{}]", str::from_utf8(utf8_arr).unwrap());
    lib::RVExtensionVersion(buffer, utf8_arr.len() + 1);
    println!("buffer result: [{}]", str::from_utf8(utf8_arr).unwrap());
}

unsafe fn rve_get() {
    let function = CString::new("fetch").unwrap().into_raw();
    let buffer = CString::new("________________________________________________________________")
        .unwrap()
        .into_raw();

    let utf8_arr: &[u8] = CStr::from_ptr(buffer).to_bytes();
    println!("buffer: [{}]", str::from_utf8(utf8_arr).unwrap());

    let mut args = Vec::new();
    let arg1 = CString::new("https://hc-dashboard-back.blackfoot.dev/")
        .unwrap()
        .into_raw();
    let arg2 = CString::new("2 argument").unwrap().into_raw();
    args.push(arg1 as *const i8);
    args.push(arg2 as *const i8);
    let argv = args.as_ptr();
    lib::RVExtensionArgs(buffer, utf8_arr.len() + 1, function, argv, args.len());
    println!("buffer result: [{}]", str::from_utf8(utf8_arr).unwrap());
}
