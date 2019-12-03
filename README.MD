Simple TCP server and client
===

A simple TCP server and client in Rust. Made it for a class.

Both client and server accepts two parameters - ip and port. `cargo run -- 127.0.0.1 10123` should launch it just fine in debug mode.

Latest commit has a version of the server that has a multi-threaded server that shares a mutex-locked object (homework requirement). See older commit for a version that runs in a single thread.
