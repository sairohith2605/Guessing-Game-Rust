use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println! ("Welcome to the Guessing Game!");
    println! ("Please input a number between 1 and 100.");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1,101);

    io::stdin().read_line(&mut guess).expect("Could not read the input.");
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("You've guessed a little lower! The right answer is {}", secret_number),
        Ordering::Greater => println! ("You've guessed a little more! The right answer is {}", secret_number),
        Ordering::Equal => println!("You've guessed it right, it's {}!", secret_number)
    }
}
