use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    let mut guess: String = String::new();
    let target = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number between 1 and 100");
    println!("Type 'q' to quit the game");

    loop {
        print!("Your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess = guess.trim();

        if guess == "q" {
            println!("You quit the game.");
            break;
        }

        match guess.parse::<u32>() {
            Ok(n) if n == target => {
                println!("You guessed correctly!");
                break;
            },
            Ok(n) if n < target => println!("Your guess was too low."),
            Ok(n) if n > target => println!("Your guess was too high."),
            Ok(_) => println!("Unknown error."),
            Err(_) => println!("Invalid input. Please enter a number between 1 and 100 or quit with 'q'."),
        }
    }
}
