use std::io;
use std::io::{BufReader, BufRead};
use std::fs;
use std::fs::File;
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

fn get_user_guess() -> String {
    loop {
        let mut guess = String::new();
        let wordle_words_file_path = "wordle-words.txt";

        println!("Enter your guess: ");

        let stdin = io::stdin();
        stdin.read_line(&mut guess)
            .expect("Failed to read line");

        if guess.len() != 6 {
            println!("Please enter a 5 letter word");
            continue;
        }

        match find_word_in_file(wordle_words_file_path, &guess) {
            Ok(found) => {
                if found {
                    return guess;
                } else {
                    println!("That is not a valid word.");
                    continue;
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn find_word_in_file(file_path: &str, word_to_find: &str) -> io::Result<bool> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim() == word_to_find.trim() {
            return Ok(true);
        }
    }

    Ok(false)
}

fn game(wordle: &str) {
    let mut guess_count = 1;
    println!("Wordle!");
    loop {
        let guess = get_user_guess();

        // TODO: remove
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

