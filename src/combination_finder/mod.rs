use std::sync::mpsc::{Receiver, Sender};

use crate::Config;

enum State {
    Idle,
    Starting,
    Running,
    Stopped
}

struct CombinationFinder<'a> {
    config: &'a Config,
    dictionary: Vec<String>,
    rx: Receiver<State>,
    tx: Sender<Vec<String>>,
    state: State
}

impl<'a> CombinationFinder<'a> {
    pub fn new(config: &'a Config, dictionary: Vec<String>, rx: Receiver<State>, tx: Sender<Vec<String>>) -> Self {
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

    }

    fn is_valid(combination: &Vec<String>, comparator: &Vec<char>) -> bool {
        let mut combination = combination.join("").chars().collect::<Vec<char>>();
        combination.sort();
        comparator.eq(&combination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![String::from("this"), String::from("is"), String::from("test")], vec!['e', 'h', 'i', 'i', 's', 's', 's', 't', 't', 't'], true)]
    #[test_case(vec![String::from("this"), String::from("is"), String::from("test")], vec!['e', 'i', 'i', 'i', 's', 's', 's', 't', 't', 't'], false)]
    fn validate_returns_expected(combination: Vec<String>, comparator: Vec<char>, expected: bool) {
        assert_eq!(CombinationFinder::is_valid(&combination, &comparator), expected);
    }
}
