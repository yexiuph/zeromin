use std::ffi::c_char;

// You need to have a pointer for this to store in memory in C++
#[repr(C)]
pub struct ZRijndael {
    pub ran_file_version: i32,
    pub rijn_version: *const c_char,
}

#[no_mangle]
pub extern "C" fn get_file_version(rijndael: &ZRijndael) -> i32 {
    // Return the file version
    rijndael.ran_file_version
}

#[no_mangle]
pub extern "C" fn get_rijndael_version(rijndael: &ZRijndael) -> *const c_char {
    rijndael.rijn_version
}