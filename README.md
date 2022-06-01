
# git-excess

Extra (excessive?) git tooling

  

# Git_utils from AppCove

This suite of tools is built using Rust, we are going through how to install it and how to use Cargo.

  

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#####  To learn about Rust i advice [the official online book](https://doc.rust-lang.org/book/) or [learning-rust.github.io](https://learning-rust.github.io/)

## Cargo

#####  The tool that helps you to manage your Rust project

- Create a new project: `cargo new <package-name>`

- Create a new project in an existing directory: `cargo init`

- Build the project: `cargo build` [debug] or `cargo build --release` [release]

- Analyze the project to see it has any errors, without building it: `cargo check`

- Run the project: `cargo run` or `cargo run --release`

- Update project dependencies: `cargo update`

- Run tests: `cargo test`

- Run benchmarks: `cargo bench`

  

## ❗

**This project is using Rust's workspaces**; workspaces are defined in the `Cargo.toml` in the root folder of the project. Each additional package is a new implementation of an AppCove's git command, it has to be added to the previously mentioned `Cargo.toml` and it's folder has to be created using `cargo new --vcs none <package-name>` (`--vcs none` don't initialize a git repo inside the package directory).

Instead, if a new library of shared code has to be created, use: `cargo new --vcs none --lib <lib-name>`
> **Note:** to run a package or build a specific one use: `cargo run <package-name>` or `cargo build <package-name>`. This if you are in root folder, if CWD is inside a package use the normal run or buid command

  

##### Advantages of using workspaces

- Packages can share multiple local libraries

- Multiple sections of a project can be managed in a structured way. All the binaries are going to be found in the `/target` folder at the root of the project.

- Even though we are working with multiples packages all of their binaries are stored in a single location and using the name of the <package-name>

