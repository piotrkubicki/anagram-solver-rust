use std::sync::{mpsc::Receiver, Mutex, Arc}; use itertools::Itertools;
use md5;

use crate::{State, Password};

pub struct PermutationsFinder {
    passwords: Arc<Mutex<Vec<Password>>>,
    data_rx: Receiver<Vec<String>>,
    state: State,
}


impl PermutationsFinder {
    pub fn new(passwords: Arc<Mutex<Vec<Password>>>, data_rx: Receiver<Vec<String>>) -> Self {
        Self {
            passwords,
            data_rx,
            state: State::Idle
        }
    }

    pub fn run(&mut self) -> Vec<String> {
        self.state = State::Starting;
        info!("PermutationsFinder running...");
        let mut result = vec![];
        self.state = State::Running;
        loop {
            if let Ok(combination) = self.data_rx.try_recv() {
                if let Some(phrase) = self.find(combination) {
                    info!("Password found: {}", phrase);
                    result.push(phrase);
                }
            }
            if result.len() == self.passwords.lock().unwrap().len() { break }
        }
        self.state = State::Stopped;
        info!("PermutationsFinder stopped!");
        result
    }

    fn find(&mut self, combination: Vec<String>) -> Option<String> {
        for perm in combination.iter().permutations(combination.len()).unique() {
            let phrase = perm.iter().copied().join(" ");
            let digest = md5::compute(&phrase);
            for password in &mut *self.passwords.lock().unwrap() {
                if password.found == false && format!("{:x}", digest).eq(&password.digest) {
                    password.phrase = phrase.clone();
                    password.found = true;
                    return Some(phrase)
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc};
    use md5;

    use super::*;

    #[test]
    fn test_find_create_expected_permutations() {
        let (_, in_rx) = mpsc::channel();
        let passwords = Arc::new(Mutex::new(vec![Password::new(format!("{:x}", md5::compute(b"this is password"))), Password::new(format!("{:x}", md5::compute(b"not a password")))]));
        let mut permutations_finder = PermutationsFinder::new(passwords, in_rx);

        let res = permutations_finder.find(vec!["password".to_string(), "is".to_string(), "this".to_string()]).unwrap();

        assert_eq!(res, "this is password");
    }

    #[test]
    fn run_stops_after_all_passwords_found() {
        let (in_tx, in_rx) = mpsc::channel();
        let passwords = Arc::new(Mutex::new(vec![Password::new(format!("{:x}", md5::compute(b"this is password"))), Password::new(format!("{:x}", md5::compute(b"yet another password")))]));
        let mut permutations_finder = PermutationsFinder::new(passwords, in_rx);
        let combinations = vec![
            vec!["some".to_string(), "just".to_string(), "words".to_string()],
            vec!["is".to_string(), "this".to_string(), "password".to_string()],
            vec!["yet".to_string(), "password".to_string(), "another".to_string()],
        ];

        for combination in combinations {
            let _ = in_tx.send(combination);
        }
        let _ = permutations_finder.run();

        assert_eq!(permutations_finder.state, State::Stopped);
    }
}
