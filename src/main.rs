use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Error;
use std::sync::Arc;
use std::sync::Mutex;
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
    let min_word_len = 3;
    let max_word_len = 10;
    let max_len = 18;
    let max_words = 3;
    let passwords = vec![
        b"e4820b45d2277f3844eac66c903e84be",
        b"23170acc097c24edb98fc5488ab033fe",
        b"665e5bcb0c20062fe8abaaf4628bb154"
    ];
    let args: Vec<String> = env::args().collect();
    if let Ok(dictionary) = Dictionary::new(&args[1], min_word_len, max_word_len) {
        log::info!("Dictionary loaded successfully!");
        let combination_length_gen = CombinationGenerator::new(max_len, min_word_len, max_word_len, max_words);
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
            let words: Vec<Vec<String>> = vec![];
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
            // let comb_tx_cp = comb_tx.clone();
            // let config = Arc::clone(&config_sync);
            // thread::spawn(move || {
            //     CombinationFinder::new(config, words, comb_tx_cp);
            // });
        }

    } else {
        Error::new(io::ErrorKind::InvalidInput, "Cannot process dictionary!");
        return Ok(());
    }


    Ok(())
}
