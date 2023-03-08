use std::io;
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
    let mut guess_count = 1;
    println!("Wordle!");
    loop {
        let mut guess = String::new();

        println!("Enter your guess: ");

        let stdin = io::stdin();
        stdin.read_line(&mut guess)
            .expect("Failed to read line");

        if guess.len() != 6 { // TODO: why 6? 
            println!("Please enter a 5 letter word");
            continue;
        }

        // TODO: ensure the word is a word in the dictionary

        for (i, letter) in wordle.chars().enumerate() {
            println!("{} {}", i, letter);
        }

        println!("[{}] {}", guess_count, guess); // TODO: color the letters based on wordle rules

        if guess.trim() == wordle {
            println!("You win! The word was {}", wordle.black().on_green());
            break;
        }

        guess_count += 1;

        if guess_count >  5 {
            println!("You lose! The word was {}", wordle);
            break;
        }
    }
}

