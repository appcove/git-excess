# git-excess
Extra (excessive?) git tooling

# Git_utils from AppCove
This suit of tools is built using Rust, we are going throught how to install it and how to use Cargo.

## Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
######  To learn about Rust i advice [the official online book](https://doc.rust-lang.org/book/) or [learning-rust.github.io](https://learning-rust.github.io/)
## Cargo 
###### The tool that helps you to manage your Rust project
Create a new project: `cargo new`
Create a new project in an existing directory: `cargo init`
Build the project: `cargo build` [debug] or `cargo build --release` [release]
Analyze the project to see it has any errors, without building it: `cargo check`
Run the project: `cargo run` or `cargo run --release`
Update project dependencies: `cargo update`
Run tests: `cargo test`
Run benchmarks: `cargo bench`


## ❗
The plan for the future is to have multiple binary inside the same directory all sharing the same library of git commands. Those binary are going to be defined in the `cargo.toml` file with a dedicated name and a path to the source code under the tag `[[bin]]`. [LEARN MORE ABOUT TAGS](https://doc.rust-lang.org/cargo/reference/manifest.html)
When this is going to happen the commands will change into `cargo run --bin <name_bin>` of `cargo build --bin <name_bin>`

