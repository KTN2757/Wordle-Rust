use colored::*;
use rand::prelude::IndexedRandom;
use rand::rng;
use std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    // Fetch the word.
    let wordle_words = fs::read_to_string("/mnt/Bulk Drive/Coding/LearnRust/wordle_words.txt");
    let wordle_words = wordle_words.unwrap();
    let mut word_list = Vec::new();
    for words in wordle_words.split_whitespace() {
        word_list.push(words);
    }
    let word = word_selector(word_list);

    println!("Welcome to Wordle! You have 6 attempts.\n");

    // Game loop
    for attempt in 1..=6 {
        println!("Attempt {}/6:", attempt);

        // Take input.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input.len() != 5 {
            println!("Please enter a 5-letter word!\n");
            continue;
        }

        // Main logic.
        let word_map: Vec<char> = word.chars().collect();
        let input_map: Vec<char> = input.chars().collect();

        let mut letter_counts: HashMap<char, usize> = HashMap::new();
        for &ch in &word_map {
            *letter_counts.entry(ch).or_insert(0) += 1;
        }

        let mut result = vec![ColorState::Gray; 5];
        for i in 0..5 {
            if input_map[i] == word_map[i] {
                result[i] = ColorState::Green;
                *letter_counts.get_mut(&input_map[i]).unwrap() -= 1;
            }
        }

        for i in 0..5 {
            if result[i] == ColorState::Gray {
                if let Some(count) = letter_counts.get_mut(&input_map[i]) {
                    if *count > 0 {
                        result[i] = ColorState::Yellow;
                        *count -= 1;
                    }
                }
            }
        }

        print!("  ");
        for i in 0..5 {
            let letter = input_map[i].to_string();
            match result[i] {
                ColorState::Green => {
                    print!("{}", letter.black().on_bright_green().bold());
                }
                ColorState::Yellow => {
                    print!("{}", letter.black().on_bright_yellow().bold());
                }
                ColorState::Gray => {
                    print!("{}", letter.white().on_bright_black().bold());
                }
            }
            print!(" ");
        }
        println!("\n");

        if input == word {
            println!(
                "Congratulations! You guessed it in {} attempt{}!",
                attempt,
                if attempt == 1 { "" } else { "s" }
            );
            return;
        }
    }

    println!("Out of attempts! The word was: {}", word.to_uppercase());
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ColorState {
    Green,
    Yellow,
    Gray,
}

// Random word selector.
fn word_selector(word_list: Vec<&str>) -> String {
    let mut rng = rng();
    if let Some(choice) = word_list.choose(&mut rng) {
        return choice.to_string();
    }
    String::from("error")
}
