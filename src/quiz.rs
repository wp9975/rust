use std::io::{self, prelude::*};

pub fn run() {
    let quiz = vec![
        ("What is the capital of France?", "a"),
        ("What is the color of the sky?", "b"),
        ("What is 2 + 2?", "c"),
        ("What is the first letter of the alphabet?", "a"),
        ("Who is the current president?", "d"),
    ];

    let mut score = 0;
    for (question, answer) in quiz {
        println!("{}", question);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim();

        if user_input == answer {
            println!("Correct!\n");
            score += 1;
        } else {
            println!("Wrong! The correct answer was {}\n", answer);
        }
    }

    println!("Your final score is: {}", score);
}

