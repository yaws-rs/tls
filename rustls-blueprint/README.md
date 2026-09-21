# Rustls Blueprint

This crate provides Rustls Blueprint

This Blueprint is typically and primarily used in `std` / operating system context or where global allocator is available.

It is currently possible but non-trivial to use Rustls without `std` (where Rustls still requires a global allocator) through RustCrypto and mbedtls etc. providers but it can be non-trivial to wire up the required environmental constraints such as random number generator and global allocator through the he use of something like embedded-alloc to provide a global allocator (with it's given trade offs and limited scope outside embedded target audience).

See [more](https://github.com/yaws-rs/tls) information.
