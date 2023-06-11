use std::io;
use std::collections::HashSet;

pub struct Hangman {
    word: Vec<char>,
    guessed: HashSet<char>,
    attempts: usize,
}

impl Hangman {
    pub fn new(word: &str) -> Hangman {
        Hangman {
            word: word.chars().collect(),
            guessed: HashSet::new(),
            attempts: 7,
        }
    }

    fn is_finished(&self) -> bool {
        self.attempts == 0 || self.word.iter().all(|c| self.guessed.contains(c))
    }

    pub fn play(&mut self) {
        while !self.is_finished() {
            println!("Attempts remaining: {}", self.attempts);
            for c in &self.word {
                print!("{}", if self.guessed.contains(c) { *c } else { '_' });
            }
            println!();

            println!("Enter a guess:");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).unwrap();
            let guess = guess.chars().nth(0).unwrap();

            if self.word.contains(&guess) {
                self.guessed.insert(guess);
            } else {
                self.attempts -= 1;
            }
        }

        if self.attempts > 0 {
            println!("Congratulations, you won!");
        } else {
            println!("Game over, the word was: {}", self.word.iter().collect::<String>());
        }
    }
}
