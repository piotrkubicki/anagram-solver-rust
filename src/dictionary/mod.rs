use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;


pub struct Dictionary {
    words: HashMap<usize, Vec<String>>
}

impl Dictionary {
    pub fn new(file_path: &str) -> Self {
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
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

            Dictionary {
                words: dictionary
            }
        } else {
            Dictionary {
                words: HashMap::new()
            }
        }
    }

    pub fn get(self, key: &usize) -> Option<Vec<String>> {
        match self.words.get(key) {
            Some(list) => Some(list.clone()),
            _ => None
        }
    }
}
