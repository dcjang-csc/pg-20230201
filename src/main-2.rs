use rand::Rng;
use std::io;

fn main(){
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("secret_number: {secret_number}");
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_ok) => println!("OK"),
        Err(_err) => println!("Err")
    };
    println!("guess {guess}");
}