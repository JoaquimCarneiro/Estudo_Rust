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
    //println!("Secret: {secret}");
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
