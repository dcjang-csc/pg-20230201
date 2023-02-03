use std::io;
use rand::{self, Rng};
use std::cmp;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("please type a number!");
    println!("guess: {guess}");

    match guess.cmp(&secret_number) {
        cmp::Ordering::Less => println!("Too small"),
        cmp::Ordering::Greater => println!("Too big"),
        cmp::Ordering::Equal => println!("You win!"),
    }
    println!("secre number: {secret_number}");
}
