use std::fs;
use rand::Rng;

fn main() {

    let wordle_words = fs::read_to_string("wordle-words.txt")
        .expect("Unable to read file");

    // random line (12971 lines in file)
    let word_line = rand::thread_rng().gen_range(0..12971);

    let wordle = wordle_words.lines().nth(word_line).unwrap();

    println!("the word to guess is {}", wordle); // TODO: remove this line
}
