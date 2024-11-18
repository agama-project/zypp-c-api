## Rust Library for Zypp

Goal of these libraries is to provide minimal API for Agama to install system. It is not aimed to provide generic zypp bindings.

### Structure of Directory

- zypp-agama-sys - low level unsafe FFI crate
- zypp-agama - safe layer on top of FFI bindings
- zypp-agama-example - example and testing binary to try API and demonstrate it
