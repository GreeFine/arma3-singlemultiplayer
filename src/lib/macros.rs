#[macro_export]
macro_rules! str_to_cstr {
    ($convert:expr) => {
        CString::new($convert).unwrap().into_raw();
    };
}
