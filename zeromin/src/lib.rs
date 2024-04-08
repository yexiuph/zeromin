#![allow(unused_imports, dead_code, unused_mut)]
extern crate tokio;
extern crate winapi;

mod engine;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::engine::crypto::zerosha;

#[no_mangle]
pub async fn async_operation_callback(cha_name: String, seconds: u64) -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
    format!(
        "Hello {} from rust!, I've waited {} seconds",
        cha_name, seconds
    )
}

#[no_mangle]
pub extern "C" fn perform_async_operation(
    char_name: *const c_char,
    seconds: u64,
    callback: extern "C" fn(*const c_char, usize),
) {
    // We would spawn a thread that would automatically kill/destroy itself after the function.
    let cha_name_str: String = unsafe { CStr::from_ptr(char_name).to_string_lossy().into_owned() };

    std::thread::spawn(move || {
        // After that we would use this tokio runtime to add async capabilities.
        // Note : In older toolchain of C++ you don't have async and await capabilities but in newer like 17+ there is.
        let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        // As you can see here we are expecting a "RUST" string in the output of the callback
        // However that is not compatible with C++
        let output: String =
            rt.block_on(async { async_operation_callback(cha_name_str, seconds).await });

        // So here we would convert it to CString so we can have the output printed to C++
        let c_string: CString = CString::new(output).expect("Failed to convert string to CString");

        // Clone in rust means copy like in C++ pointers.
        let c_string_clone: CString = c_string.clone();

        // Here we converted the CString into i8 so we can pass the callback from our async function.
        let result: *mut i8 = c_string.into_raw();

        // This is the output
        //  Here we would get the length of the data
        callback(result, c_string_clone.as_bytes().len());

        // Ensure that the memory is properly manage
        unsafe {
            let _ = CString::from_raw(result);
        };
    });
}
