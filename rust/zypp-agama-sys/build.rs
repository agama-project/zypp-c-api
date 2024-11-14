fn main() {
    println!("cargo::rustc-link-search=native=../c-layer/");
    println!("cargo::rustc-link-lib=static=agama-zypp");
    println!("cargo::rustc-link-lib=dylib=zypp");
    println!("cargo::rustc-link-lib=dylib=stdc++");
}
