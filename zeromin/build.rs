extern crate cbindgen;

use std::env;

fn main() {
    // Generate the C header file
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::Cxx;
    
    let binder = cbindgen::generate_with_config(&crate_dir, config);
    match binder {
        Ok(binding) => {
            binding.write_to_file("../target/zeromin/zerominffi.h");
        }
        Err(_err) => {}
    }
}
