use std::env;
use std::io;

use combination_generator::{CombinationGenerator};
use dictionary::{Dictionary};

mod combination_generator;
mod dictionary;


#[derive(Clone)]
pub struct Config {
    min_word_len: usize,
    max_word_len: usize,
    target_len: usize,
    max_words: usize,
}

impl Config {
    pub fn new(min_word_len: usize, max_word_len: usize, target_len: usize, max_words: usize) -> Self {
        Config {
            min_word_len,
            max_word_len,
            target_len,
            max_words
        }
    }

}

fn main() -> io::Result<()> {
    let config:  Config = Config::new(3, 10, 18, 4);
    let args: Vec<String> = env::args().collect();
    if let Ok(dictionary) = Dictionary::new(&args[1], &config) {
        println!("{:?}", dictionary.get(&3));
    }

    // let mut combination_length_gen = CombinationGenerator::new(config);

    // while let Some(combination) = combination_length_gen.next() {
    //     println!("{:?}", combination);
    // }

    Ok(())
}
