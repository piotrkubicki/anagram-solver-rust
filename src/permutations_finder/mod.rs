use std::sync::mpsc::{Receiver, Sender};
use itertools::Itertools;
use md5;

use crate::{Command, State};

pub struct PermutationsFinder {
    passwords: Vec<String>,
    data_rx: Receiver<Vec<String>>,
    comm_tx: Sender<Command>,
    state: State,
}

impl PermutationsFinder {
    pub fn new(passwords: Vec<String>, data_rx: Receiver<Vec<String>>, comm_tx: Sender<Command>) -> PermutationsFinder {
        PermutationsFinder {
            passwords,
            data_rx,
            comm_tx,
            state: State::Idle
        }
    }

    pub fn run(&mut self) {
        self.state = State::Starting;
        log::info!("PermutationsFinder running...");
        let mut counter: usize = 0;
        self.state = State::Running;
        loop {
            if let Ok(combination) = self.data_rx.try_recv() {
                if let Some(phrase) = self.find(combination) {
                    counter += 1;
                    log::info!("Password found: {}", phrase);
                }
            }
            if counter >= self.passwords.len() { break }
        }
        self.state = State::Stopped;
        let _ = self.comm_tx.send(Command::Stop);
        log::info!("PermutationsFinder stopped!");
    }

    fn find(&self, combination: Vec<String>) -> Option<String> {
        for perm in combination.iter().permutations(combination.len()).unique() {
            log::info!("{:?}", perm);
            let phrase = perm.iter().copied().join(" ");
            let digest = md5::compute(&phrase);
            for password in &self.passwords {
                if format!("{:x}", digest).eq(password) { return Some(phrase) }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use md5;

    use super::*;

    #[test]
    fn test_find_create_expected_permutations() {
        let (_, in_rx) = mpsc::channel();
        let (comm_tx, _) = mpsc::channel();
        let passwords = vec![format!("{:x}", md5::compute(b"this is password")), format!("{:x}", md5::compute(b"not a password"))];
        let permutations_finder = PermutationsFinder::new(passwords, in_rx, comm_tx);

        let res = permutations_finder.find(vec!["password".to_string(), "is".to_string(), "this".to_string()]).unwrap();

        assert_eq!(res, "this is password");
    }

    #[test]
    fn run_stops_after_all_passwords_found() {
        let (in_tx, in_rx) = mpsc::channel();
        let (comm_tx, _) = mpsc::channel();
        let passwords = vec![format!("{:x}", md5::compute(b"this is password")), format!("{:x}", md5::compute(b"yet another password"))];
        let mut permutations_finder = PermutationsFinder::new(passwords, in_rx, comm_tx);
        let combinations = vec![
            vec!["some".to_string(), "just".to_string(), "words".to_string()],
            vec!["is".to_string(), "this".to_string(), "password".to_string()],
            vec!["yet".to_string(), "password".to_string(), "another".to_string()],
        ];

        for combination in combinations {
            let _ = in_tx.send(combination);
        }
        permutations_finder.run();

        assert_eq!(permutations_finder.state, State::Stopped);
    }
}
