use std::{env, path::Path, process::Command};
use bindgen::builder;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut cmd = Command::new("make");
    cmd.arg("-C");
    cmd.arg(Path::new(&manifest_dir).join("../../c-layer").as_os_str());
    if let Err(e) = cmd.output() {
        panic!("Building C library failed: {}\n", e.to_string().as_str());
    }

    let bindings = builder()
        .header("headers.h")
        .merge_extern_blocks(true)
        .clang_arg("-I")
        .clang_arg("../../c-layer/include")
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/bindings.rs").expect("Couldn't write bindings!");

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
