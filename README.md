Now it is just POC to have thin C layer only for agama purpose on top of libzypp.

### Repo Organization

Repository contains three directories:

- c-layer that contain sources to build static library and in include directory C ready headers.
  The internal directory contains C++ headers that is used only for inter-communication and is private.
- c-example is example pure C binary that is using static library and produce some output
- rust is target rust code that should be used from Agama, but it is not ready yet.

### Libzypp Notes

- libzypp is not thread safe
- for seeing how it works see yast2-pkg-bindings and zypper as some parameters in calls are ignored
- goal is to have thin layer close to libzypp and build logic on top of it in more advanced language

### Interesting Resources

- https://doc.rust-lang.org/nomicon/ffi.html
- https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
- https://www.khoury.northeastern.edu/home/lth/larceny/notes/note7-ffi.html
- https://cliffle.com/blog/not-thread-safe/ ( interesting part how to ensure in rust that some data is not thread safe )
