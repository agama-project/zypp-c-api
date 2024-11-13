Now it is just POC to have thin C layer only for agama purpose on top of libzypp.

### Libzypp Notes

- libzypp is not thread safe
- for seeing how it works see yast2-pkg-bindings and zypper as some parameters in calls are ignored
- goal is to have thin layer close to libzypp and build logic on top of it in more advanced language

### Interesting Resources

- https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
- https://www.khoury.northeastern.edu/home/lth/larceny/notes/note7-ffi.html
