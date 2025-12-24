# Getting started with Rust programming language

## 0 - About this

This git repository is simply my personal notes, as I introduce my self to the Rust programming language.

## 1 - Getting Started

### 1.1 - Installing Rust

#### 1.1.1 - Official installer (Linux / macOS)

If you are using Linux or macOS you can use the official installer using a terminal and curl:

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

#### 1.1.2 - Linux

As always, you should use your package manager to install software:

##### 1.1.2.1 - Debian, Ubuntu, etc.

`sudo apt install rust`

##### 1.1.2.2 - Archlinux

The two main ways to install Rust are:

- The Native installation, recommended if you only use Rust for building or installing software made with Rust
- The Rustup installation, recommended if you intend to program anything in Rust

If you opt by going with the native installation, you should at least install:

- rust - package containing the compiler, cargo, and other base tools
- rust-src - Source code for the Rust standard library

`sudo pacman -S rust rust-src`

The minimal requirements to start, is to install the package `rust`. It contains the basic tools needed to start programming in Rust.

If you choose to go with the `rustup` option, first you need to install it:

`sudo pacman -S rustup`

and then chose between stable, beta, nightly or a numbered version, like 1.83.0:

`rustup default stable`

`rustup toolchain install stable`



#### 1.1.3 - macOS

Rust can also be installed by using [homebrew](https://brew.sh/) in a terminal:

`brew install rust`

#### 1.1.4 - Windows

Users of windows should already know how to install software... ❤️😚

#### 1.1.5 - The linker

Rust also needs a linker in Linux and macOS. This can be achieved by installing a C compiler like GCC or Clang. 

Usually, in Linux, an C compiler is already installed. In a terminal type `gcc --version` or `clang --version` to check if it is installed. If you get an error, proceed installing GCC or Clang.

In macOS a C compiler can be obtained by using xcode in a terminal:

`xcode-select --install`

[todo]

- updating rust

### 1.2 - Hello World...

```rust
fn main () {
    println!("Hello? is it me you are looking for?");
}
```

[todo]

- File creation convention 
- code description
- compile procedure using rustc

### 1.3 - Compiling with rustc

### 1.4 - Cargo

[todo]

- cargo description and uses

## 2 - Basic Example - Guessing game

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut guess_n = 0;
    let max_guess = 5;

    println!("######## Guessing Game! ########");
    println!(
        "Guess a number between 1 and 100.\nYou have {} guesses!\n################################\n",
        max_guess
    );
    println!("Secret: {secret}");
    loop {
        if guess_n >= max_guess {
            println!("###################\n#### You loose ####\n###################");
            break;
        }
        println!(
            "#### Try: {} - {} to go ####",
            guess_n + 1,
            max_guess - guess_n
        );

        println!("Insert a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, please");
                continue;
            }
        };
        //println!("your guess: {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!("--> Too small"),
            Ordering::Greater => println!("--> Too big"),
            Ordering::Equal => {
                println!("##################\n#### You win! ####\n##################");
                break;
            }
        }
        guess_n += 1;
    }
}
```



## 3 - Basic concepts

This kind of concepts, like defining variables, functions and control flow, are common to most programming languages.

As in other languages, Rust has a set o reserved keywords, that can not be used for naming variables, constants or functions.

### 3.1 - Variables, constants and mutability

#### 3.1.1 - Variables and mutability

Variables are defined using the keyword `let`:

```rust
let x = 5;
```

By default variables in Rust are immutable.

This code will **fail** to compile:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

If a value of a variable needs to change at runtime, it has to be declared as mutable, using the keyword `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

#### 3.1.2 - constants

Like immutable variables, constants are not allowed to change, bat have a few differences:

- Can't use `mut`. They are always immutable!
- Uses the keyword `const`.
- The type has to be annotated.
- Can be declared in any scope, including the global scope.
- The value must be from a constant expression computed at compile time, not a value computed at runtime.

Example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Constants are valid for the entire time a program runs, within the scope in which they were declared.

#### 3.1.3 - shadowing

Unlike most other programming languages Rust let's you declare a new variable with the same name as a previous variable. This is known as shadowing. This means that the second variable "overshadows" the first, taking to it self any uses of that variable name until itself is shadowed or it scope ends. Example:

```rust
fn main() {
	let z = 5;
    let z = z + 1; // z is shadowed and it's 6 now
    {
        let z = z * 2; // in this scope z is shadowed and it's 12 now
        println!("The value of z in the inner scope is: {z}");
    }
    // outside the previous scope the value returns to 6
    println!("The value of x is: {z}");
}
```

This is different from using `mut`. In fact, shadowing is reassigning a new variable, allowing a few transformations and even change the type of the value assigned to the variable. For instance:

```rust
    let spaces = "   "; // spaces is a String
    let spaces = spaces.len(); // spaces is a Integer 
```

using `mut` in this case would result in a compile error:

```rust
    let mut spaces = "   ";
    spaces = spaces.len(); // this will throw an error!
```

### 3.2 - Data Types



## A - Bibliography and resources

- [Rust language website](https://rust-lang.org/)
- [Arch Linux Wiki](https://wiki.archlinux.org/title/Rust)
