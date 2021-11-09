From the [Rust Learning Path](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/) on Microsoft Learn

## Introduction
* Rust is a safe alternative to  C and C++. Like C and C++
* Rust doesn't have a large runtime or garbage collector
* Rust prevents bugs from incorrect memory use > C and C++

### Advantages
* Type Safe: Compiler ensures all variables have correct type
* Memory Sare: References always refer to valid memory
* Date Race Free: Borrow prevents multi*mutate an variable
* Zero-cost Absctraction: High-level features have low cost
* Minimal Runtime: Small runtime and no garbage collector
* Targets Bare Metal: Suitable for OS and Device Drivers

### Rust Module System
Rust manage and orgalize code with the following items:
* Crates: The smallest piece of code the compiler can run
* Modules: Manage the scope of items in side a crate
* Path: For naming items in code and contral privacy

### Std and 3rd-party Libs
Standard library and other 3rd-party libs from [crates.io](https://crates.io/):
* std: The standard library with following modules:
    + std::collections - For collection types, such as HashMap.
    + std::env - Work with system environment.
    + std::fmt - Control output format.
    + std::fs - Work with the file system.
    + std::io - Work with input/output.
    + std::path - Work with file system path data.
* structopt - 3rd-party crate for parsing command-line arguments.
* chrono - 3rd-party crate to handle date and time data.
* regex - 3rd-party crate to work with regular expressions.
* serde - 3rd-party crate to se-deserialize data structures.
Import modules with `use std::fmt` or `use regex::Regex`.

### Manage Project
Create and manage project, and dependencies with Cargo:
* cargo new: Create new project
* cargo build: Build a project
* cargo run: Build and Run a project
* cargo test: Test a project
* cargo check: Check project types
* cargo doc: Build documentation for a project
* cargo publish: Publish a library to crates.io
* cargo.toml: Manage dependent crates

### Rust Playground
Use [it](https://play.rust-lang.org/) to try some code, check syntax, or share code:
There are some build in tools and dev features:
* Format Code: Rustfmt tool can auto format code style
* Test Code: Clippy tool run *lint* to find errors
* Save Code: Auto saved in browser local storage
* Share Code: Create GitHub gist for sharing
Provide the following build options:
* Run: Build and Run the code = cargo run
* Build: Build but don't run = cargo build
* Test: Build and run Test = cargo test
There are also some limitations:
* Network: when compile or run, network connection down
* Memory: available memory is limited for compile and run
* Execute Time: compile and run may timeout if too long
* Disk: disk space for compile and run is also limited
Time to Play:
1. Go to [Rust Playground](https://play.rust-lang.org/)
2. Enter the following code:
`fn main(){println!(Welcome to Rust!);}`
3. Format the code: Tools > Rustfmt
4. Check mistakes: Tools > Clippy
5. Fix it with adding quote marks:
```
fn main() {
    println!("Welcome to Rust!");
}
```
6. Build and Run by choosing *RUN*
7. Click *SHARE* and use **Permalink to the playground**