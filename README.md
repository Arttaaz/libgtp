# libgtp
library for the Go Text Protocol in Rust

It contains an implementation of a controller as well, but you can opt out of it if you need to make your own by using no-default-features in your Cargo.toml.
Only the controller use the std crate, so as long as you have an allocator appropriate for your platform, you can use all the types, command and response handling without the std.
