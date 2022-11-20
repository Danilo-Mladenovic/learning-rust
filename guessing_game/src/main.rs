use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess:");

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failet to read line.");

        let user_guess: i32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {user_guess}");

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
