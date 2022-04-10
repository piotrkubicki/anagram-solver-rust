use std::collections::VecDeque;

#[derive(Clone)]
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
    pub fn new(config: Config) -> Self {
        let state = vec![config.min_word_len; config.max_words as usize];
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
            if *value < self.config.max_word_len {
                *value += 1;
                return Ok(());
            } else {
                *value = 1;
            }
        }
        self.state = vec![];
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

pub struct CombinationGenerator {
    generators: VecDeque<Generator>,
    seen_combinations: Vec<Vec<u32>>,
}

impl CombinationGenerator {
    pub fn new(config: Config) -> Self {
        let mut generators = VecDeque::new();
        for max_words in 1..config.max_words + 1 {
            if max_words * config.max_word_len >= config.target_len {
                let mut config = config.clone();
                config.max_words = max_words;
                generators.push_back(
                    Generator::new(config)
                );
            }
        }

        CombinationGenerator {
            generators,
            seen_combinations: vec![],
        }
    }
}

impl Iterator for CombinationGenerator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut generator) = self.generators.pop_front() {
            while let Some(mut combination) = generator.next() {
                combination.sort();
                if !self.seen_combinations.contains(&combination) {
                    self.seen_combinations.push(combination.clone());
                    self.generators.push_back(generator);
                    return Some(combination);
                }
            };
        };

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
        let mut generator = Generator::new(config);
        generator.state = state;

        let _ = generator.increment();

        assert_eq!(expected, generator.state);
    }

    #[test_case(vec![1, 1, 1], false)]
    #[test_case(vec![5, 3, 2], true)]
    #[test_case(vec![4, 4, 2], true)]
    #[test_case(vec![1, 4, 2], false)]
    fn is_valid_returns_expected(state: Vec<u32>, expected: bool) {
        let config = Config::new(2, 6, 10, 3);
        let mut generator = Generator::new(config);
        generator.state = state;

        assert_eq!(generator.is_valid(), expected);
    }

    #[test]
    fn get_next_combination_returns_valid_combination_lengths() {
        let config = Config::new(3, 10, 21, 3);
        let mut generator = Generator::new(config);

        assert_eq!(generator.next().unwrap(), vec![10, 8, 3]);
        assert_eq!(generator.next().unwrap(), vec![9, 9, 3]);
    }

    #[test]
    fn combination_generator_calls_generators_in_turns() {
        let config = Config::new(2, 6, 8, 3);
        let mut combination_generator = CombinationGenerator::new(config);

        assert_eq!(combination_generator.next().unwrap().len(), 2);
        assert_eq!(combination_generator.next().unwrap().len(), 3);
        assert_eq!(combination_generator.next().unwrap().len(), 2);
    }

    #[test]
    fn combination_generator_returns_expected_number_of_combinations() {
        let config = Config::new(3, 10, 18, 4);
        let mut combination_length_gen = CombinationGenerator::new(config);
        let mut combinations = vec![];

        while let Some(combination) = combination_length_gen.next() {
            combinations.push(combination);
        }

        assert_eq!(combinations.len(), 21);
    }
}
