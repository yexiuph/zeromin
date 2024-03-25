use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct ZeroPath {
    _antizero: [u8; 0],// Ensure the struct is not zero-sized and stays in the memory
    pub glogic: *const c_char,
    pub glevel: *const c_char,
    pub gnpctalk: *const c_char,
    pub gquest: *const c_char,
    pub ganimation: *const c_char,
    pub geffect: *const c_char,
    pub gchareffect: *const c_char,
    pub gskinobject: *const c_char,
}

impl ZeroPath {
    // Function to initialize ZeroPath struct
    pub fn new() -> ZeroPath {
        let base_path = "./zero";
        let game_logic_path = "/gamelogic";
        let game_files_path = "/gamefiles";
        ZeroPath {
            _antizero: [],
            glogic: ptr_from_string(&(base_path.to_string() + game_logic_path + "/glogic.rcc")),
            glevel: ptr_from_string(&(base_path.to_string() + game_logic_path + "/level.rcc")),
            gnpctalk: ptr_from_string(&(base_path.to_string() + game_logic_path + "/npctalk.rcc")),
            gquest: ptr_from_string(&(base_path.to_string() + game_logic_path + "/quest.rcc")),
            ganimation: ptr_from_string(&(game_files_path.to_string() + "/animation.rcc")),
            geffect: ptr_from_string(&(game_files_path.to_string() + "/effect.rcc")),
            gchareffect: ptr_from_string(&(game_files_path.to_string() + "/effectchar.rcc")),
            gskinobject: ptr_from_string(&(game_files_path.to_string() + "/skinobject.rcc")),
        }
    }
}

// Helper function to convert &str to *const c_char
fn ptr_from_string(s: &str) -> *const c_char {
    CString::new(s).unwrap().into_raw()
}

// Function to deallocate memory of ZeroPath struct
#[no_mangle]
extern "C" fn zeropath_delete(zp: *mut ZeroPath) {
    unsafe {
        if !zp.is_null() {
            // Convert raw pointers back to CString and deallocate memory
            let zp = zp.as_mut().unwrap();
            let _ = CString::from_raw(zp.glogic as *mut _);
            let _ = CString::from_raw(zp.glevel as *mut _);
            let _ = CString::from_raw(zp.gnpctalk as *mut _);
            let _ = CString::from_raw(zp.gquest as *mut _);
            let _ = CString::from_raw(zp.ganimation as *mut _);
            let _ = CString::from_raw(zp.geffect as *mut _);
            let _ = CString::from_raw(zp.gchareffect as *mut _);
            let _ = CString::from_raw(zp.gskinobject as *mut _);
        }
    }
}

// Function to initialize ZeroPath struct
#[no_mangle]
extern "C" fn zeropath_init() -> *mut ZeroPath {
    Box::into_raw(Box::new(ZeroPath::new()))
}