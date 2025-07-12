use std::io::{self, Write};
use std::process::Command;

use rand::Rng;

const ATTEMPTS: u8 = 3;
const MIN_RANGE: u8 = 0;
const MAX_RANGE: u8 = 99;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}

fn pause_until_any_key() {
    let mut buffer = String::new();
    print!("Premi invio per continuare...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn tell_rules() {
    loop {
        clear_screen();
        
        println!("Welcome on the NUMBER GAME!");
        pause_until_any_key();
        clear_screen();
        println!("The game will choose a number and then you will have {} attempts to guess the number.", ATTEMPTS);
        pause_until_any_key();
        clear_screen();
        println!("With each attempt you'll get an hint.");
        pause_until_any_key();
        clear_screen();
        println!("Did you understand the rules? (yes/y)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        let answer = answer.trim().to_lowercase();

        if answer == "yes" || answer == "y" {
            clear_screen();
            println!("Great! Let's start!");
            pause_until_any_key();
            clear_screen();
            break;
        }
        
        clear_screen();
        println!("Let's repeat the rules...");
        pause_until_any_key();
        clear_screen();
    }
}

fn check_attempts_left(attempts_left: u8) -> bool {
    attempts_left > 0
}

fn get_attempt(value: u8, guess: u8) -> String {
    if value > guess { "higher".to_string() } else { "lower".to_string() }
}

fn main() {
    tell_rules();

    let mut attempts_left = ATTEMPTS;
    let mut current_guess: Option<u8> = None;
    let mut value_to_guess: u8 = 0;

    value_to_guess = rand::rng().random_range(MIN_RANGE..MAX_RANGE+1);
    loop {
        clear_screen();
        if !check_attempts_left(attempts_left) {
            println!("It will be for next time...");
            pause_until_any_key();
            break;
        }

        println!("ATTEMPTS LEFT: {}", attempts_left);
        if let Some(guess) = current_guess {
            println!("Hint: the value to guess is {}", get_attempt(value_to_guess, guess));
        }
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        current_guess = answer.trim().parse::<u8>().ok();

        if let Some(guess) = current_guess {
            if guess != value_to_guess {
                attempts_left -= 1;
                continue;
            }
            println!("Congrats! You won!!!");
            pause_until_any_key();
        }

        break;
    }
}