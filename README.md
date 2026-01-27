# learn-rust

## Installing Rust

### Linker

In macOS use Xcode Command Line Tools to install a C compiler first. And before you start itching, Rust is not a flavor of C. It requires though a linker, which is a program that combines object files into an executable.

```
xcode-select --install
```

### Rust on macOS

Use Homebrew to install Rust. But install rustup first. Then run `rustup-init` to install Rust, as well as the Rust toolchain, which includes the package manager `cargo`.

```
brew install rustup
rustup-init
```

This will launch a prompt and you can safely pick option number 1. Wait for the installation to complete, and then close an open your terminal.
You can also update Rust by running `rustup update`.

### Zed

You will need a code editor, although 99% will definitely use VS Code, I prefer Zed. You can download it from [here](https://zed.dev/download). YOu can use it free of charge, but it has an affordable subscription model for advanced features such as better access to Claude.

### The Brown Book

The book is the default starting point for learning Rust. It covers the basics of the language and provides a solid foundation for building more complex applications. But Brown University has a great version of the book available for free online here: [https://www.brown.edu/academics/computer-science/rust-book](https://www.brown.edu/academics/computer-science/rust-book).

## Setting up Cargo

You have probably started creating a new repository in Github. Clone it to your machine and run this command to initialize a new Rust project:

```
cargo init
```

Compile the project:

```
cargo build
```

Run the project:

```
cargo run
```
