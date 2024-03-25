use std::slice;

const BLOCKSIZE: usize = 64;
const SHA256LENGTH: usize = 8;

// Internal helpers
fn circular_shift(ui_bits: u32, ui_word: u32) -> u32 {
    (ui_word << ui_bits) | (ui_word >> (32 - ui_bits))
}
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & (y ^ z)) ^ z
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x|y)&z)|(x&y)
}

fn sig0(x: u32) -> u32 {
    ((x >> 2)|(x << 30)) ^ ((x >> 13)|(x << 19)) ^ ((x >> 22)|(x << 10))
}

fn sig1(x:u32) -> u32 {
    ((x >> 6)|(x << 26)) ^ ((x >> 11)|(x << 21)) ^ ((x >> 25)|(x << 7))
}

fn sig2(x: u32) -> u32 {
    ((x >> 7)|(x << 25)) ^ ((x >> 18)|(x << 14)) ^ (x >> 3)
}

fn sig3(x: u32) -> u32 {
    ((x >> 17)|(x << 15)) ^ ((x >> 19)|(x << 13)) ^ (x >> 10)
}

fn bytes_to_word(pc_bytes: &[u8]) -> u32 {
    (pc_bytes[3] as u32) | ((pc_bytes[2] as u32) << 8) | ((pc_bytes[1] as u32) << 16) | ((pc_bytes[0] as u32) << 24)
}

fn word_to_bytes(mut rui_word: u32, pc_bytes: &mut [u8]) {
    for i in 0..4 {
        pc_bytes[i] = (rui_word & 0xFF) as u8;
        rui_word >>= 8;
    }
}