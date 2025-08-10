use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let random_number = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();

        print!("Guess a number: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error while reading input");

        // Shadowing "guess" string variable with an unsigned 32-bit integer variable.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number between 1-100.");
                continue;
            },
        };

        match guess.cmp(&random_number) {
            Ordering::Greater => println!("{guess} is too high."),
            Ordering::Less => println!("{guess} is too low."),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            },
        }
    }
}

