use std::io;
use std::io::Write;
use rand::prelude::*;

fn main() {
    let mut guess: String = String::new();
    let target: u8 = random();

    while guess.trim() != "q" {
        guess.clear();

        println!("Guess a number between {} and {}", u8::MIN, u8::MAX);
        println!("(q)uit the program");

        print!("Your guess: ");
        io::stdout().flush().expect("Cannot flush stdout");

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read user input");

        println!();

        match guess.trim().parse::<u8>() {
            Ok(n) if n == target => {
                println!("You guessed correct!");
                break;
            },
            Ok(n) if n < target => println!("Your guess was lower"),
            Ok(n) if n > target => println!("Your guess was higher"),
            Ok(_) => println!("Not sure what is going on here..."),
            Err(e) => println!("Error: {}", e),
        }
    }
}
