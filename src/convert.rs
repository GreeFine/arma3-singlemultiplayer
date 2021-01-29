use std::ffi::CStr;

// get str from arma
pub unsafe fn arma_str(a3str: *const i8) -> String {
    let utf8_arr: &[u8] = CStr::from_ptr(a3str).to_bytes();
    String::from_utf8(utf8_arr.into()).unwrap()
}
