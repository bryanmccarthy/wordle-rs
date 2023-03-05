use std::fs;
use rand::Rng;
use colored::*;

fn main() {

    let wordle_words = fs::read_to_string("wordle-words.txt")
        .expect("Unable to read file");

    // random line (12971 lines in file)
    let word_line = rand::thread_rng().gen_range(0..12971);

    let wordle = wordle_words.lines().nth(word_line).unwrap();

    println!("the word to guess is {}", wordle); // TODO: remove this line

    game(wordle);
}

fn game(wordle: &str) {
    let mut guess = String::new();
    let guess_count = 0;

    println!("Wordle!");
    loop {
        println!("Enter your guess: ");

        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // TODO: ensure the guess is a 5 letter word that is in the wordle_words.txt file
        
        println!("[{}] {}", guess_count, guess); // TODO: color the letters based on wordle rules
        
        if guess.trim() == wordle {
            println!("The word was {}", wordle.black().on_green());
            break;
        }
    }
}
