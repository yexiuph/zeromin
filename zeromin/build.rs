extern crate cbindgen;

use std::env;

use cbindgen::SortKey;

fn main() {
    println!("cargo:rerun-if-changed=src");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.cpp_compat = true;
    config.tab_width = 4;
    config.pragma_once = true;
    config.sort_by = SortKey::Name;
    config.language = cbindgen::Language::Cxx;

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("../ffi/zerominffi.h");

    // let output_dir = Path::new(&crate_dir).join("../target/zeromin");
    // let target_dir = env::var("OUT_DIR").unwrap();
    // let target_dir = Path::new(&target_dir);

    // let mut src_paths = vec![
    //     target_dir.join("debug"), // for debug builds
    //     target_dir.join("release"), // for release builds
    // ];

    // // Find the binary files with .lib or .exe extension
    // let binary_files: Vec<_> = src_paths
    //     .iter_mut()
    //     .flat_map(|dir| {
    //         fs::read_dir(dir)
    //             .unwrap()
    //             .filter_map(|entry| entry.ok())
    //             .filter(|entry| {
    //                 entry.file_name().to_string_lossy().ends_with(".lib")
    //                     || entry.file_name().to_string_lossy().ends_with(".exe")
    //             })
    //             .map(|entry| entry.path())
    //             .collect::<Vec<_>>()
    //     })
    //     .collect();

    // for src_path in binary_files {
    //     let file_name = src_path.file_name().unwrap();
    //     let dest_path = output_dir.join(file_name);
    //     fs::copy(&src_path, &dest_path).unwrap();
    // }
}
