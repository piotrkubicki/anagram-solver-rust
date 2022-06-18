use std::sync::mpsc::{Receiver, Sender};

use crate::{Config, dictionary};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Idle,
    Starting,
    Running,
    Stopped
}

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

struct CombinationFinder<'a> {
    config: &'a Config,
    dictionary: Vec<Vec<String>>,
    rx: Receiver<State>,
    tx: Sender<Vec<String>>,
    state: State
}

impl<'a> CombinationFinder<'a> {
    pub fn new(config: &'a Config, dictionary: Vec<Vec<String>>, rx: Receiver<State>, tx: Sender<Vec<String>>) -> Self {
        CombinationFinder {
            config,
            dictionary,
            rx,
            tx,
            state: State::Idle
        }
    }

    pub fn run(&mut self) {
        self.state = State::Starting;
        self.find_combinations(SimpleFinder{});
        println!("CombinationFinder finished!");
    }

    fn is_valid(combination: &Vec<String>, comparator: &Vec<char>) -> bool {
        let mut combination = combination.join("").chars().collect::<Vec<char>>();
        combination.sort();
        comparator.eq(&combination)
    }

    fn find_combinations<T: Finder>(&mut self, finder: T) {
        let mut counter = DictionaryIterator::new(&self.dictionary);
        self.state = State::Running;
        while let Some(c) = counter.next() {
            if let Ok(msg) = self.rx.try_recv() {
                println!("Stop command received! Stopping...");
                break;
            }
            let words = finder.find(c, &self.dictionary);
            self.tx.send(words);
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
    use core::time;
    use std::{sync::mpsc, thread};

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
        assert_eq!(CombinationFinder::is_valid(&combination, &comparator), expected);
    }

    #[test]
    fn run_return_expected_combinations() {
        let config = Config::new(3, 5, 8, 3);
        let dictionary = vec![
            vec![String::from("who"), String::from("bet"), String::from("set")],
            vec![String::from("test"), String::from("best")],
            vec![String::from("dizzy"), String::from("junky"), String::from("zippy"), String::from("quack")],
        ];
        let (_, rx_state) = mpsc::channel();
        let (tx_res, rx_res) = mpsc::channel();
        let mut combination_finder = CombinationFinder::new(&config, dictionary, rx_state, tx_res);
        assert_eq!(combination_finder.state, State::Idle);
        combination_finder.run();

        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("test"), String::from("dizzy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("test"), String::from("junky")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("test"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("test"), String::from("quack")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("best"), String::from("dizzy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("best"), String::from("junky")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("best"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("who"), String::from("best"), String::from("quack")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("test"), String::from("dizzy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("test"), String::from("junky")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("test"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("test"), String::from("quack")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("best"), String::from("dizzy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("best"), String::from("junky")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("best"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("bet"), String::from("best"), String::from("quack")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("test"), String::from("dizzy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("test"), String::from("junky")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("test"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("test"), String::from("quack")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("best"), String::from("dizzy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("best"), String::from("junky")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("best"), String::from("zippy")]);
        assert_eq!(rx_res.try_recv().unwrap(), vec![String::from("set"), String::from("best"), String::from("quack")]);
        assert_eq!(combination_finder.state, State::Stopped);
    }

    #[test]
    fn find_combination_stops_when_stop_signal_received() {
        struct MockFinder {}

        impl Finder for MockFinder {
            fn find(&self, combination: Vec<isize>, dictionary: &Vec<Vec<String>>) -> Vec<String> {
                thread::sleep(time::Duration::from_millis(10));
                vec!["".to_string()]
            }
        }

        let config = Config::new(3, 5, 8, 3);
        let dictionary = vec![
            vec![String::from("who"), String::from("bet"), String::from("set")],
            vec![String::from("test"), String::from("best")],
            vec![String::from("dizzy"), String::from("junky"), String::from("zippy"), String::from("quack")],
        ];
        let (tx_state, rx_state) = mpsc::channel();
        let (tx_res, rx_res) = mpsc::channel();
        let mut combination_finder = CombinationFinder::new(&config, dictionary, rx_state, tx_res);

        combination_finder.find_combinations(MockFinder{});
        tx_state.send(State::Stopped);

        assert_eq!(combination_finder.state, State::Stopped);
        let mut combinations: Vec<Vec<String>> = vec![];
        for combination in rx_res.try_recv() {
            combinations.push(combination);
        }

        assert!(combinations.len() < 10);
    }
}
