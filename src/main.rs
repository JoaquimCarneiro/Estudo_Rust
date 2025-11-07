use std::io;

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Guessing Game!");
    println!("Insert a number: ");

    let mut guess = String::new();
    println!("Secret: {secret}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("your guess: {guess}");
}
