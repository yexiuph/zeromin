mod engine;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub async fn async_operation_callback(seconds: u64) -> String {
    // Simulate asynchronous work (e.g., network request, I/O operation)
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
    format!("Hello from rust!, I've waited {} seconds", seconds)
}

#[no_mangle]
pub extern "C" fn perform_async_operation(seconds: u64, callback: extern "C" fn(*const c_char, usize)) {
    std::thread::spawn(move || {
        let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        
        let output: String = rt.block_on(async {
            async_operation_callback(seconds).await
        });

        let c_string: CString = CString::new(output).expect("Failed to convert string to CString");
        let c_string_clone: CString = c_string.clone();

        let result: *mut i8 = c_string.into_raw();
        
        // This is the output
        callback(result, c_string_clone.as_bytes().len());

        // Ensure that the memory is properly managed
        unsafe { let _ = CString::from_raw(result); };
    });
}