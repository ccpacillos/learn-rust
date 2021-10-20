use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! (from 1-100)");
    println!("Enter your guess:");

    let random_number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("To big!"),
        Ordering::Equal => println!("You got the correct number!"),
    }
}
