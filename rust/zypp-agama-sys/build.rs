use std::{env, path::Path, process::Command};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut cmd = Command::new("make");
    cmd.arg("-C");
    cmd.arg(Path::new(&manifest_dir).join("../../c-layer").as_os_str());
    if let Err(e) = cmd.output() {
        panic!("Building C library failed: {}\n", e.to_string().as_str());
    }

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
