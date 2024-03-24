use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use std::sync::Mutex as RustMutex;
use std::fs::File;
use std::io::Read;

// // Define FileStream struct
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct FileStream {
//     pub file_name: *mut c_char, // Use *mut c_char for compatibility with C strings
//     pub file_content: *mut Vec<u8>, // Use *mut Vec<u8> for compatibility with C pointers
// }

// // Define YXUnzip struct
// #[repr(C)]
// #[derive()]
// pub struct YXUnzip {
//     pub files: *mut RustMutex<Vec<FileStream>>, // Use *mut Mutex<Vec<FileStream>> for compatibility with C pointers
// }

// // Define new function for YXUnzip
// #[no_mangle]
// pub extern "C" fn yxunzip_new() -> *mut YXUnzip {
//     let unzipper = YXUnzip {
//         files: Box::into_raw(Box::new(RustMutex::new(Vec::new()))),
//     };
//     Box::into_raw(Box::new(unzipper))
// }

// // Define load_rcc function for YXUnzip
// #[no_mangle]
// pub extern "C" fn yxunzip_load_rcc(unzipper_ptr: *mut YXUnzip, zip_path: *const c_char) -> bool {
//     unsafe {
//         let unzipper = &*unzipper_ptr;
//         let zip_path_str = CStr::from_ptr(zip_path).to_str().expect("Invalid UTF-8 string");
//         let zip_path = zip_path_str;

//         let zip_file = File::open(zip_path).expect("Failed to open file");
//         let mut archive = zip::ZipArchive::new(zip_file).expect("Failed to open archive");

//         for i in 0..archive.len() {
//             let mut file = archive.by_index(i).expect("Failed to get file from archive");
//             let mut content = Vec::new();
//             file.read_to_end(&mut content).expect("Failed to read file content");

//             let file_name = CString::new(file.name()).expect("Failed to convert file name to CString").into_raw();
//             let file_content = Box::into_raw(Box::new(content));

//             let file_info = FileStream {
//                 file_name,
//                 file_content,
//             };

//             let mut files_lock = (*unzipper.files).lock().expect("Failed to acquire lock");
//             files_lock.push(file_info);
//         }
//     }
//     true
// }