use std::{error::Error, ffi::CStr};

// get str from arma
pub unsafe fn arma_str<'a>(a3str: *const i8) -> Result<&'a str, Box<dyn Error>> {
    let utf8_arr: &[u8] = CStr::from_ptr(a3str).to_bytes();
    let convertion = std::str::from_utf8(utf8_arr.into());
    let result = convertion?;
    Ok(result.trim_matches('"'))
}
