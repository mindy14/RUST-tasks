extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Guess a number please");

    let rand_num = rand::thread_rng().gen_range(0, 1000);

    loop {
        println!("Your Guess: ");

        let mut guess_num = String::new();

        io::stdin().read_line(&mut guess_num)
            .expect("Error: Line not read");

        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess_num);

        match guess_num.cmp(&rand_num) {
            Ordering::Less    => println!("Lesser than the secret number"),
            Ordering::Greater => println!("Greater than the secret number"),
            Ordering::Equal   => {
                println!("Correct! You WON!");
                break;
            }
        }
    }
}