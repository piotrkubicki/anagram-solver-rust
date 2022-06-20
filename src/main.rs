use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Error;
use std::sync::mpsc;
use std::thread;
use itertools::Itertools;
use log;

use dictionary::Dictionary;
use combination_generator::CombinationGenerator;
use combination_finder::CombinationFinder;
use permutations_finder::PermutationsFinder;

mod combination_finder;
mod combination_generator;
mod dictionary;
mod permutations_finder;


#[derive(Clone)]
pub struct Config {
    min_word_len: usize,
    max_word_len: usize,
    target_len: usize,
}

impl Config {
    pub fn new(min_word_len: usize, max_word_len: usize, target_len: usize) -> Self {
        Config {
            min_word_len,
            max_word_len,
            target_len,
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Stop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Idle,
    Starting,
    Running,
    Stopped
}

fn main() -> io::Result<()> {
    let config:  Config = Config::new(3, 10, 18);
    let max_words = 3;
    let passwords = vec![b"e4820b45d2277f3844eac66c903e84be", b"23170acc097c24edb98fc5488ab033fe", b"665e5bcb0c20062fe8abaaf4628bb154"];
    let args: Vec<String> = env::args().collect();
    if let Ok(dictionary) = Dictionary::new(&args[1], &config) {
        log::info!("Dictionary loaded successfully!");
        let combination_length_gen = CombinationGenerator::new(&config, max_words);
        // let (comm_tx, comm_rx) = mpsc::channel();
        // let (comb_tx, comb_rx) = mpsc::channel();
        // let (perm_tx, perm_rx) = mpsc::channel();
        // thread::spawn(|| {
        //     CombinationFinder::new(&config, wl_rx, comb_tx);
        // });

        // thread::spawn(|| {
        //     PermutationsFinder::new(comb_rx, comm_tx);
        // });

        for combination in combination_length_gen {
            let mut words: Vec<Vec<String>> = vec![];
            for word_len in combination {
                if let Some(words_list) = dictionary.get(&word_len) {
                    let mut list: Vec<String> = vec![];
                    for word in words_list {
                        list.push(word.clone());
                    }
                } else {
                    log::error!("Selected word length not found!");
                    continue;
                }
            }
        }

    } else {
        Error::new(io::ErrorKind::InvalidInput, "Cannot process dictionary!");
        return Ok(());
    }


    Ok(())
}
