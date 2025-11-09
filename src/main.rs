use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guessing Game!");
    println!("Secret: {secret}");

    loop {
        println!("Insert a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Not a number !");

        println!("your guess: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
