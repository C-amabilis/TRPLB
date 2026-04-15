use std::io;

use rand::Rng;



pub fn start() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("What is your guess?");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("The number was {secret_number}");
    println!("Your guess was {guess}");
}
