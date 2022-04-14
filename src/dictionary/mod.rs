use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error};
use std::fs::File;

use crate::Config;


pub struct Dictionary {
    config: Config,
    words: HashMap<usize, Vec<String>>
}

impl Dictionary {
    pub fn new(file_path: &str, config: Config) -> Result<Self, Error> {
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let dictionary = Dictionary::map(&mut reader);
        Ok(Dictionary {
            config: config,
            words: dictionary,
        })
    }

    pub fn get(self, key: &usize) -> Option<Vec<String>> {
        match self.words.get(key) {
            Some(list) => Some(list.clone()),
            _ => None
        }
    }

    fn map<T: BufRead>(reader: &mut T) -> HashMap<usize, Vec<String>> {
        let mut dictionary: HashMap<usize, Vec<String>> = HashMap::new();

        for line in reader.lines() {
            if let Ok(word) = line {
                let word_len = word.len();
                if dictionary.contains_key(&word_len) {
                    if let Some(list) = dictionary.get_mut(&word_len) {
                        list.push(word);
                    }
                }
                else {
                    dictionary.insert(word_len, vec![word]);
                }
            }
        }
        dictionary
    }
}
