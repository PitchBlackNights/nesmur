//! Temporary module test

/// Temporary function test
fn main() {
    // I have to do this so I can generate documentation for the main app.
    // If I don't separate the main app from `main.rs` then the binary
    // documentation will overwrite the library documentation.
    nesmur::app::main();
}
