use std::io;

fn main() {
    println!("Guessing Game!");
    println!("Insert a number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("your guess: {guess}");
}
