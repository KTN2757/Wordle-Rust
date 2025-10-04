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
    println!("{word}");

    // Take input.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("fizz");
    let input = input.trim();

    // Main logic.
    if input.len() == 5 {
        let word_map: Vec<char> = word.chars().collect();
        let input_map: Vec<char> = input.chars().collect();
        println!("{:?}", word_map);
        println!("{:?}", input_map);

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

        for i in 0..5 {
            let letter = input_map[i].to_string();
            match result[i] {
                ColorState::Green => {
                    println!("{}", String::from(input_map[i]).green().on_yellow());
                }
                ColorState::Yellow => {
                    println!("{}", String::from(input_map[i]).black().on_white());
                }
                ColorState::Gray => {
                    println!("{}", String::from(input_map[i]).red());
                }
            }
        }
    } else {
        println!("Please enter a 5-letter word!");
    }
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
    String::from("Error")
}
