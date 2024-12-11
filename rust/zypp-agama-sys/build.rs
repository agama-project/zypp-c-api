use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
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
