use std::env;
use std::io;
use std::io::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use dictionary::Dictionary;

use combination_generator::CombinationGenerator;
use combination_finder::CombinationFinder;
use itertools::Itertools;
use permutations_finder::PermutationsFinder;

mod combination_finder;
mod combination_generator;
mod dictionary;
mod permutations_finder;

#[macro_use] extern crate log;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Idle,
    Starting,
    Running,
    Stopped
}

pub struct Password {
    phrase: String,
    digest: String,
    found: bool,
}

impl Password {
    fn new(digest: String) -> Self {
        Password{
            phrase: String::new(),
            digest,
            found: false,
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();
    info!("Starting");
    let min_word_len = 2;
    let max_word_len = 10;
    let target = 8;
    let max_words = 4;
    let min_word_len_mutex = Arc::new(Mutex::new(min_word_len));
    let max_word_len_mutex = Arc::new(Mutex::new(max_word_len));
    let passwords = Arc::new(Mutex::new(vec![
        Password::new("e4820b45d2277f3844eac66c903e84be".to_string()),
        Password::new("23170acc097c24edb98fc5488ab033fe".to_string()),
        Password::new("665e5bcb0c20062fe8abaaf4628bb154".to_string()),
    ]));
    let passwords_cp = passwords.clone();
    let passwords_cp2 = passwords.clone();
    let comparator = vec!["poultry", "outwits", "ants"].join("").chars().collect::<Vec<char>>();
    let args: Vec<String> = env::args().collect();
    let max_workers = 6;

    if let Ok(dictionary) = Dictionary::new(&args[1], *Arc::clone(&min_word_len_mutex).lock().unwrap(), *Arc::clone(&max_word_len_mutex).lock().unwrap(), &comparator) {
        info!("Dictionary loaded successfully!");
        let (comb_tx, comb_rx) = mpsc::channel();

        let mut combination_length_gen = CombinationGenerator::new(
            target,
            min_word_len,
            max_word_len,
            max_words
        );
        let (tx, rx) = mpsc::channel();
        let thread_no = Arc::new(Mutex::new(0));
        let thread_no_cp = thread_no.clone();
        let thread_no_val = thread_no.clone();
        thread::spawn(move || {
             loop {
                 if *thread_no_cp.lock().unwrap() < max_workers {
                     if let Some(combination) = combination_length_gen.next() {
                         info!("Combination {:?}", combination);
                         let mut words: Vec<Vec<String>> = vec![];
                         for word_len in combination {
                             if let Some(words_list) = dictionary.get(&word_len) {
                                 let mut list: Vec<String> = vec![];
                                 for word in words_list {
                                     list.push(word.clone());
                                 }
                                 words.push(list);
                             } else {
                                 error!("Selected word length {} not found!", word_len);
                                 continue;
                             }
                         }
                         let comb_tx_cp = comb_tx.clone();
                         let comparator = comparator.clone();
                         let tx = tx.clone();
                         let _ = thread::spawn(move || {
                             let _ = tx.send(CombinationFinder::new(words, comb_tx_cp, comparator).run());
                         });
                         *thread_no.lock().unwrap() += 1;
                     }
                 }
                 if let Ok(_) = rx.try_recv() {
                     *thread_no.lock().unwrap() -= 1;
                 }

                 if *thread_no.lock().unwrap() == 0 {
                     break;
                 }
            }
        });

        let (tx, rx) = mpsc::channel();
        let tx = tx.clone();
        thread::spawn(move || {
            let mut permutations_finder = PermutationsFinder::new(passwords_cp, comb_rx);
            let _ = tx.send(permutations_finder.run());
        });

        loop {
            thread::sleep(Duration::new(5, 0));
            if let Ok(_) = rx.try_recv() {
                break;
            }
            else if *thread_no_val.lock().unwrap() == 0 {
                break;
            }
        }
        let result = passwords_cp2.lock().unwrap();
        let passwords = result.iter().map(|password| {password.phrase.clone()}).collect_vec();
        info!("Found passwords: {:?}", passwords);
    } else {
        Error::new(io::ErrorKind::InvalidInput, "Cannot process dictionary!");
        return Ok(());
    }

    Ok(())
}
