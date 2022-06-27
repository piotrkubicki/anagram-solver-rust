use std::{sync::mpsc::Sender, fmt::Error};

use crate::State;

struct DictionaryIterator {
    max_values: Vec<isize>,
    state: Vec<isize>
}

impl DictionaryIterator {
    fn new(dictionary: &Vec<Vec<String>>) -> Self {
        let mut max_values: Vec<isize> = vec![];
        for list in dictionary {
            max_values.push((list.len() - 1) as isize)
        }
        let mut state: Vec<isize> = vec![];
        for i in 0..dictionary.len() {
            if i < dictionary.len() - 1 {
                state.push(0);
            } else {
                state.push(-1)
            }
        }

        DictionaryIterator {
            max_values,
            state,
        }
    }
}

impl Iterator for DictionaryIterator {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..self.max_values.len()).rev() {
            if self.state.get(i) < self.max_values.get(i) {
                if let Some(value) = self.state.get_mut(i) {
                    *value += 1;
                    return Some(self.state.clone());
                }
            } else {
                if let Some(value) = self.state.get_mut(i) {
                    *value = 0;
                }
            }
        }
        None
    }
}

pub struct CombinationFinder {
    dictionary: Vec<Vec<String>>,
    tx: Sender<Vec<String>>,
    comparator: Vec<char>,
    state: State
}

impl CombinationFinder {
    pub fn new(dictionary: Vec<Vec<String>>, tx: Sender<Vec<String>>, comparator: Vec<char>) -> Self {
        CombinationFinder {
            dictionary,
            tx,
            comparator,
            state: State::Idle
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.state = State::Starting;
        self.comparator.sort();
        self.find_combinations(SimpleFinder{});
        info!("CombinationFinder finished!");
        Ok(())
    }

    fn is_valid(&self, combination: &Vec<String>) -> bool {
        let mut combination = combination.join("").chars().collect::<Vec<char>>();
        combination.sort();
        self.comparator.eq(&combination)
    }

    fn find_combinations<T: Finder>(&mut self, finder: T) {
        info!("Finder is running");
        let mut counter = DictionaryIterator::new(&self.dictionary);
        self.state = State::Running;
        while let Some(c) = counter.next() {
            let words = finder.find(c, &self.dictionary);
            if self.is_valid(&words) {
                // info!("{:?} is valid", words);
                let _ = self.tx.send(words);
            }
        }
        self.state = State::Stopped;
    }
}

trait Finder {
    fn find(&self, combination: Vec<isize>, dictionary: &Vec<Vec<String>>) -> Vec<String>;
}

struct SimpleFinder {}

impl Finder for SimpleFinder {
    fn find(&self, combination: Vec<isize>, dictionary: &Vec<Vec<String>>) -> Vec<String> {
        let mut words: Vec<String> = vec![];
        for (i, item) in combination.iter().enumerate() {
            if let Some(word_list) = dictionary.get(i) {
                if let Some(word) = word_list.get(*item as usize) {
                    words.push(word.to_string());
                }
            }
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use crate::State;

    use super::*;
    use test_case::test_case;

    #[test_case(3, 3, 27)]
    #[test_case(2, 3, 8)]
    #[test_case(1, 8, 1)]
    #[test_case(8, 1, 8)]
    fn test_dictionary_iterator_creates_expected_number_of_combinations(rows: usize, columns: usize, expected_combinations: isize) {
        let mut counter = 0;
        let iterator = DictionaryIterator::new(&vec![vec!["a".to_string(); rows]; columns]);

        for _ in iterator {
            counter += 1;
        }

        assert_eq!(counter, expected_combinations);
    }

    #[test]
    fn test_dictionary_iterator() {
        let dictionary = vec![
            vec![String::from("who"), String::from("bet"), String::from("set"), String::from("yet")],
            vec![String::from("test"), String::from("best"), String::from("rest")],
        ];
        let mut iterator = DictionaryIterator::new(&dictionary);

        assert_eq!(iterator.next().unwrap(), vec![0, 0]);
        assert_eq!(iterator.next().unwrap(), vec![0, 1]);
        assert_eq!(iterator.next().unwrap(), vec![0, 2]);
        assert_eq!(iterator.next().unwrap(), vec![1, 0]);
        assert_eq!(iterator.next().unwrap(), vec![1, 1]);
        assert_eq!(iterator.next().unwrap(), vec![1, 2]);
        assert_eq!(iterator.next().unwrap(), vec![2, 0]);
        assert_eq!(iterator.next().unwrap(), vec![2, 1]);
        assert_eq!(iterator.next().unwrap(), vec![2, 2]);
        assert_eq!(iterator.next().unwrap(), vec![3, 0]);
        assert_eq!(iterator.next().unwrap(), vec![3, 1]);
        assert_eq!(iterator.next().unwrap(), vec![3, 2]);
        assert_eq!(iterator.next(), None);
    }

    #[test_case(vec![String::from("this"), String::from("is"), String::from("test")], vec!['e', 'h', 'i', 'i', 's', 's', 's', 't', 't', 't'], true)]
    #[test_case(vec![String::from("this"), String::from("is"), String::from("test")], vec!['e', 'i', 'i', 'i', 's', 's', 's', 't', 't', 't'], false)]
    fn validate_returns_expected(combination: Vec<String>, comparator: Vec<char>, expected: bool) {
        let dictionary = vec![];
        let (tx_res, _) = mpsc::channel();
        let combination_finder = CombinationFinder::new(dictionary, tx_res, comparator);
        assert_eq!(combination_finder.is_valid(&combination), expected);
    }

    #[test]
    fn run_return_expected_combinations() {
        let dictionary = vec![
            vec![String::from("who"), String::from("bet"), String::from("set")],
            vec![String::from("test"), String::from("best"), String::from("pies")],
            vec![String::from("dizzy"), String::from("junky"), String::from("zippy"), String::from("tyztp")],
        ];
        let (tx_res, rx_res) = mpsc::channel();
        let mut combination_finder = CombinationFinder::new(dictionary, tx_res, vec!['e', 'h', 'i', 'o', 'p', 'p', 's', 't', 't', 'w', 'y', 'z']);
        assert_eq!(combination_finder.state, State::Idle);
        let _ = combination_finder.run();

        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("test"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("pies"), String::from("tyztp")]);
        assert_eq!(combination_finder.state, State::Stopped);
    }
}
