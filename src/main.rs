use colored::*;
use rand::prelude::IndexedRandom;
use rand::rng;
use std::fs;
use std::io;

fn main() {
    // Fetch the word.
    let wordle_words = fs::read_to_string("/mnt/Bulk Drive/Coding/LearnRust/wordle_words.txt");
    let wordle_words = wordle_words.unwrap() as String;
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
        for i in 0..input_map.len() {
            if input_map[i] == word_map[i] {
                println!("{}", String::from(input_map[i]).green().on_yellow());
            } else if word.contains(input_map[i]) {
                println!("{}", String::from(input_map[i]).black().on_white());
            } else {
                println!("{}", String::from(input_map[i]).red());
            }
        }
    }
}
// Random word selector.
fn word_selector(word_list: Vec<&str>) -> String {
    let mut rng = rng();
    if let Some(choice) = word_list.choose(&mut rng) {
        return choice.to_string();
    }
    String::from("Error")
}
