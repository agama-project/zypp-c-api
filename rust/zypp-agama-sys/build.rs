use bindgen::builder;
use std::{env, path::Path, process::Command};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut cmd = Command::new("make");
    cmd.arg("-C");
    cmd.arg(Path::new(&manifest_dir).join("../../c-layer").as_os_str());
    let result = cmd.status().expect("Failed to start make process");
    if !result.success() {
        panic!("Building C library failed.\n");
    }

    let bindings = builder()
        .header("headers.h")
        .merge_extern_blocks(true)
        .clang_arg("-I")
        .clang_arg("../../c-layer/include")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    println!(
        "cargo::rustc-link-search=native={}",
        Path::new(&manifest_dir).join("../../c-layer").display()
    );
    println!("cargo::rustc-link-lib=static=agama-zypp");
    println!("cargo::rustc-link-lib=dylib=zypp");
    // NOTE: install the matching library for your compiler version, for example
    // libstdc++6-devel-gcc13.rpm
    println!("cargo::rustc-link-lib=dylib=stdc++");
}
