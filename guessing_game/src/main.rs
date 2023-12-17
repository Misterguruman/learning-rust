extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    loop {
        let secret_number: i32 = rand::thread_rng().gen_range(1..101);
        println!("DEBUG: Secret Number: {}", secret_number);

        println!("Please select a value between 0 - 100:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }

        println!("You guessed {}", guess);
        println!("The Secret Number was {}", secret_number);
    }
}
