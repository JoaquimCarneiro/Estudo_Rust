# Getting started with Rust programming language

## About this

This git repository is simply my personal notes, as I introduce my self to the Rust programming language.

## Installing Rust

### Official installer (Linux / macOS)

If you are using Linux or macOS you can use the official installer using a terminal and curl:

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

### Linux

As always, you should use your package manager to install software:

##### - Debian, Ubuntu, etc.

`sudo apt install rust`

##### - Archlinux

`sudo pacman -S rust`

### macOS

Rust can also be installed by using [homebrew](https://brew.sh/) in a terminal:

`brew install rust`

### Windows

Users of windows should already know how to install software... ❤️😚

### The linker

Rust also needs a linker in Linux and macOS. This can be achieved by installing a C compiler like GCC or Clang. 

Usually, in Linux, an C compiler is already installed. In a terminal type `gcc --version` or `clang --version` to check if it is installed. If you get an error, proceed installing GCC or Clang.

In macOS a C compiler can be obtained by using xcode in a terminal:

`xcode-select --install`

## Hello World...

```rust
fn main () {
    println!("Hello? is it me you are looking for?");
}
```

[todo]

- File creation convention 
- code description
- compile procedure using rustc

## Cargo

[todo]

- cargo description and uses

## Basic concepts

This kind of concepts, like defining variables, functions and control flow, are common to most programming languages.

As in other languages, Rust has a set o reserved keywords, that can not be used for naming variables, constants or functions.

### Variables and mutability

By default variables in Rust are immutable.

### constants



### shadowing



## Bibliography and resources

- [Rust language website](https://rust-lang.org/)
