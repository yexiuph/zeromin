extern crate cbindgen;

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.cpp_compat = true;
    config.tab_width = 4;
    config.pragma_once = true;
    config.language = cbindgen::Language::Cxx;

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("../ffi/zerominffi.h");
}
