# Cargo

Cargo is Rust's build system and package manager. Handles:

* Building your code.
* Downloading the libraries (dependencies) your code depends on.
* Building libraries code depends on.

Packages of dependencies are known as crates.

Check out this hello_world program created with cargo in hello_world_cargo.

Created with: `cargo new hello_cargo`

Cargo.toml is what lists the package version as well as the list of dependencies.
Cargo.lock is like go.sum, locking the dependency version and it will never
need to be edited manually.

## Commands:

* cargo build -- build the prog
  * cargo build --release -- build the prog in target/release with optimizations
* cargo run -- build and run the prog
* cargo check -- check program can be compiled