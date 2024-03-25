use std::os::raw::c_void;
use std::slice;

pub const BLOCKSIZE: usize = 64;

pub const SHA256LENGTH: usize = 8;

pub const SM_H256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub const SM_K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[repr(C)]
pub struct ZeroSHA {
    _antizero: [u8; 0], // Ensure the struct is not zero-sized
    pub m_aui_buf: [u32; SHA256LENGTH],
    pub m_aui_bits: [u32; 2],
    pub m_auc_in: [u8; BLOCKSIZE],
}

impl ZeroSHA {
    pub fn new() -> *mut ZeroSHA {
        let mut m_aui_buf = [0; SHA256LENGTH];
        for i in 0..SHA256LENGTH {
            m_aui_buf[i] = SM_H256[i];
        }

        let mut zerosha = Box::new(ZeroSHA { 
            _antizero: [],
            m_aui_buf,
            m_aui_bits: [0; 2],
            m_auc_in: [0; BLOCKSIZE], // Assuming m_auc_in should also be initialized to zeros
        });
        
        let zerosha_ptr = Box::into_raw(zerosha);
        zerosha_ptr
    }
}

// Implement a function to deallocate the memory when done with the struct
impl Drop for ZeroSHA {
    fn drop(&mut self) {
        // This function is called when the ZeroSHA instance goes out of scope
        // Deallocate the memory for the struct
        unsafe {
            let _ = Box::from_raw(self as *mut Self);
        }
    }
}





// Internal helpers
fn circular_shift(ui_bits: u32, ui_word: u32) -> u32 {
    (ui_word << ui_bits) | (ui_word >> (32 - ui_bits))
}
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & (y ^ z)) ^ z
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x | y) & z) | (x & y)
}

fn sig0(x: u32) -> u32 {
    ((x >> 2) | (x << 30)) ^ ((x >> 13) | (x << 19)) ^ ((x >> 22) | (x << 10))
}

fn sig1(x: u32) -> u32 {
    ((x >> 6) | (x << 26)) ^ ((x >> 11) | (x << 21)) ^ ((x >> 25) | (x << 7))
}

fn sig2(x: u32) -> u32 {
    ((x >> 7) | (x << 25)) ^ ((x >> 18) | (x << 14)) ^ (x >> 3)
}

fn sig3(x: u32) -> u32 {
    ((x >> 17) | (x << 15)) ^ ((x >> 19) | (x << 13)) ^ (x >> 10)
}


fn bytes_to_word(pc_bytes: &[u8]) -> u32 {
    (pc_bytes[3] as u32)
        | ((pc_bytes[2] as u32) << 8)
        | ((pc_bytes[1] as u32) << 16)
        | ((pc_bytes[0] as u32) << 24)
}

fn word_to_bytes(mut rui_word: u32, pc_bytes: &mut [u8]) {
    for i in 0..4 {
        pc_bytes[i] = (rui_word & 0xFF) as u8;
        rui_word >>= 8;
    }
}

// Extern Cxx
#[no_mangle]
pub extern "C" fn create_zerosha() -> *mut ZeroSHA {
    ZeroSHA::new()
}