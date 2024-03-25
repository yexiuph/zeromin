use std::ffi::c_char;

// You need to have a pointer for this to store in memory in C++
#[repr(C)]
pub struct ZRijndael {
    _antizero: [u8; 0], // Ensure the struct is not zero-sized and stays in the memory
    pub ran_file_version: i32,
    pub rijn_version: *const c_char,
}

#[repr(C)]
pub struct ZStruct {
    name: *const c_char,
    value: ZValue,
}

#[repr(C)]
pub enum ZValue {
    _Int(i32),
    _Float(f64),
}

#[no_mangle]
pub extern "C" fn data_new() -> *mut ZStruct {
    println!("{}", "Inside data_new().".to_string());

    Box::into_raw(Box::new(ZStruct {
        name: std::ffi::CString::new("ZStruct")
            .expect("Error: CString::new()")
            .into_raw(),
        value: ZValue::_Float(20000.00000),
    }))
}

#[no_mangle]
pub extern "C" fn data_free(ptr: *mut ZStruct) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
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
