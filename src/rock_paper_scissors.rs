use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn variants() -> [&'static str; 3] {
        ["rock", "paper", "scissors"]
    }

    fn from_str(input: &str) -> Option<Choice> {
        match input {
            "rock" => Some(Choice::Rock),
            "paper" => Some(Choice::Paper),
            "scissors" => Some(Choice::Scissors),
            _ => None,
        }
    }
}

pub fn play() {
    let mut rng = rand::thread_rng();

    loop {
        println!("Choose your action: Rock, Paper, Scissors");

        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice).unwrap();
        let user_choice = user_choice.trim().to_lowercase();

        let user_choice = match Choice::from_str(&user_choice) {
            Some(choice) => choice,
            None => {
                println!("Invalid choice, try again.");
                continue;
            }
        };

        let computer_choice = match rng.gen_range(0..3) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            _ => Choice::Scissors,
        };

        match user_choice {
            Choice::Rock => {
                if computer_choice == Choice::Scissors {
                    println!("You Win!");
                } else if computer_choice == Choice::Paper {
                    println!("You Lose!");
                } else {
                    println!("It's a draw.");
                }
            },
            Choice::Paper => {
                if computer_choice == Choice::Rock {
                    println!("You Win!");
                } else if computer_choice == Choice::Scissors {
                    println!("You Lose!");
                } else {
                    println!("It's a draw.");
                }
            },
            Choice::Scissors => {
                if computer_choice == Choice::Paper {
                    println!("You Win!");
                } else if computer_choice == Choice::Rock {
                    println!("You Lose!");
                } else {
                    println!("It's a draw.");
                }
            },
        }
        print!("Play again? (y/n) ");
        io::stdout().flush().unwrap();
        
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim().to_lowercase() != "y" {
            break;
        }
    }
}
