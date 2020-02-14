use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println! ("Welcome to the Guessing Game!");
    println! ("Please input a number between 1 and 100.");

    let mut guess = String::new();
    let secret_number = generate_random_number();
    io::stdin().read_line(&mut guess).expect("Could not read the input.");
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    let result: u32 = verify_guess_value(guess, secret_number);
    if result == 1 {
        println!("Sorry, you've guessed it a little less! The answer's {}", secret_number);
    } else if result == 2 {
        println!("Sorry, you've guessed it a little more! The answer's {}", secret_number);
    } else {
        println!("You're right! The secret is {}", secret_number);
    }
}

fn generate_random_number() -> u32 {
    let random_number = rand::thread_rng().gen_range(1,101);
    return random_number;
}

fn verify_guess_value(user_guess: u32, secret_number: u32) -> u32 {
    match user_guess.cmp(&secret_number) {
        Ordering::Less => return 1,
        Ordering::Greater => return 2,
        Ordering::Equal => return 3
    }
}
