pub struct Config {
    min_word_len: u32,
    max_word_len: u32,
    target_len: u32,
    max_words: u32,
}


impl Config {
    pub fn new(min_word_len: u32, max_word_len: u32, target_len: u32, max_words: u32) -> Self {
        Config {
            min_word_len,
            max_word_len,
            target_len,
            max_words
        }
    }

}

pub struct Generator {
    config: Config,
    state: Vec<u32>
}

impl Generator {
    pub fn new(config: Config, state: Vec<u32>) -> Self {
        Generator {
            config,
            state
        }
    }

    fn is_valid(&self) -> bool {
        if self.state.iter().sum::<u32>() != self.config.target_len {
            return false;
        }

        for value in self.state.iter() {
            if value > &self.config.max_word_len || value < &self.config.min_word_len {
                return false;
            }
        }
        true
    }

    fn increment<'a>(&mut self) -> Result<(), String> {
        for value in self.state.iter_mut() {
            if value < &mut self.config.max_word_len {
                *value += 1;
                return Ok(());
            } else {
                *value = 1;
            }
        }
        Err(String::from("Max combination reached!"))
    }
}

impl Iterator for Generator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.increment() == Ok(()) {
            if self.state.iter().sum::<u32>() == self.config.target_len && self.is_valid() {
                return Some(self.state.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2, 2, 2], vec![3, 2, 2])]
    #[test_case(vec![6, 2, 2], vec![1, 3, 2])]
    #[test_case(vec![6, 6, 2], vec![1, 1, 3])]
    #[test_case(vec![4, 6, 6], vec![5, 6, 6])]
    fn increment_correct_combination_element(state: Vec<u32>, expected: Vec<u32>) {
        let config = Config::new(2, 6, 10, 3);
        let mut generator = Generator::new(config, state);

        let _ = generator.increment();

        assert_eq!(expected, generator.state);
    }

    #[test_case(vec![1, 1, 1], false)]
    #[test_case(vec![5, 3, 2], true)]
    #[test_case(vec![4, 4, 2], true)]
    #[test_case(vec![1, 4, 2], false)]
    fn is_valid_returns_expected(state: Vec<u32>, expected: bool) {
        let config = Config::new(2, 6, 10, 3);
        let generator = Generator::new(config, state);

        assert_eq!(generator.is_valid(), expected);
    }

    #[test]
    fn get_next_combination() {
        let config = Config::new(3, 10, 21, 3);
        let mut generator = Generator::new(config, vec![1, 1, 1]);

        assert_eq!(generator.next().unwrap(), vec![10, 8, 3]);
        assert_eq!(generator.next().unwrap(), vec![9, 9, 3]);
    }
}
