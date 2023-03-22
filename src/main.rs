use colored::*;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    let wordle_words = fs::read_to_string("wordle-words.txt").expect("Unable to read file");

    // random line (12971 lines in file)
    let word_line = rand::thread_rng().gen_range(0..12971);

    let wordle = wordle_words.lines().nth(word_line).unwrap();

    // println!("the wordle word is {}", wordle);

    game(wordle);
}

fn get_user_guess() -> String {
    loop {
        let mut guess = String::new();
        let wordle_words_file_path = "wordle-words.txt";

        let stdin = io::stdin();
        stdin.read_line(&mut guess).expect("Failed to read line");

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

fn format_guess(guess: &str, guess_count: u8, wordle: &str) {
    let mut position_colors: [&str; 5] = ["-"; 5];

    for (i, guess_letter) in guess.chars().enumerate() {
        for (j, wordle_letter) in wordle.chars().enumerate() {
            if guess_letter == wordle_letter {
                if i == j {
                    position_colors[i] = "g";
                } else {
                    if position_colors[i] != "g" {
                        position_colors[i] = "o";
                    }
                }
            }
        }
    }

    print!("[{}]", guess_count);
    io::stdout().flush().unwrap();

    for (i, letter) in guess.chars().enumerate() {
        if position_colors[i] == "g" {
            print!("{}", letter.to_string().green());
        } else if position_colors[i] == "o" {
            print!("{}", letter.to_string().red());
        } else {
            print!("{}", letter);
        }
        io::stdout().flush().unwrap();
    }
    println!();
}

fn game(wordle: &str) {
    let mut guess_count: u8 = 1;
    println!("Wordle!\n");
    loop {
        let guess = get_user_guess();

        format_guess(guess.trim(), guess_count, wordle);

        if guess.trim() == wordle {
            println!("You win!");
            break;
        }

        guess_count += 1;

        if guess_count > 5 {
            println!("You lose! The word was {}", wordle);
            break;
        }
    }
}
