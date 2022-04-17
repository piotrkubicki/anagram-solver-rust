use std::collections::VecDeque;
use crate::Config;


pub struct Generator<'a> {
    config: &'a Config,
    max_words: usize,
    state: Vec<usize>
}

impl<'a> Generator<'a> {
    pub fn new(config: &'a Config, max_words: usize) -> Self {
        let state = vec![config.min_word_len; max_words];
        Generator {
            config,
            max_words,
            state
        }
    }

    fn is_valid(&self) -> bool {
        if self.state.iter().sum::<usize>() != self.config.target_len {
            return false;
        }

        for value in self.state.iter() {
            if value > &self.config.max_word_len || value < &self.config.min_word_len {
                return false;
            }
        }
        true
    }

    fn increment(&mut self) -> Result<(), String> {
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

impl<'a> Iterator for Generator<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.increment() == Ok(()) {
            if self.state.iter().sum::<usize>() == self.config.target_len && self.is_valid() {
                return Some(self.state.clone());
            }
        }
        None
    }
}

pub struct CombinationGenerator<'a > {
    generators: VecDeque<Generator<'a>>,
    seen_combinations: Vec<Vec<usize>>,
}

impl<'a> CombinationGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut generators = VecDeque::new();
        for max_words in 1..config.max_words + 1 {
            if max_words * config.max_word_len >= config.target_len {
                generators.push_back(
                    Generator::new(&config, max_words)
                );
            }
        }

        CombinationGenerator {
            generators,
            seen_combinations: vec![],
        }
    }
}

impl<'a> Iterator for CombinationGenerator<'a> {
    type Item = Vec<usize>;

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
    fn increment_correct_combination_element(state: Vec<usize>, expected: Vec<usize>) {
        let config = Config::new(2, 6, 10, 3);
        let mut generator = Generator::new(&config, 3);
        generator.state = state;

        let _ = generator.increment();

        assert_eq!(expected, generator.state);
    }

    #[test_case(vec![1, 1, 1], false)]
    #[test_case(vec![5, 3, 2], true)]
    #[test_case(vec![4, 4, 2], true)]
    #[test_case(vec![1, 4, 2], false)]
    fn is_valid_returns_expected(state: Vec<usize>, expected: bool) {
        let config = Config::new(2, 6, 10, 3);
        let mut generator = Generator::new(&config, 3);
        generator.state = state;

        assert_eq!(generator.is_valid(), expected);
    }

    #[test]
    fn get_next_combination_returns_valid_combination_lengths() {
        let config = Config::new(3, 10, 21, 3);
        let mut generator = Generator::new(&config, 3);

        assert_eq!(generator.next().unwrap(), vec![10, 8, 3]);
        assert_eq!(generator.next().unwrap(), vec![9, 9, 3]);
    }

    #[test]
    fn combination_generator_calls_generators_in_turns() {
        let config = Config::new(2, 6, 8, 3);
        let mut combination_generator = CombinationGenerator::new(&config);

        assert_eq!(combination_generator.next().unwrap().len(), 2);
        assert_eq!(combination_generator.next().unwrap().len(), 3);
        assert_eq!(combination_generator.next().unwrap().len(), 2);
    }

    #[test]
    fn combination_generator_returns_expected_number_of_combinations() {
        let config = Config::new(3, 10, 18, 4);
        let mut combination_length_gen = CombinationGenerator::new(&config);
        let mut combinations = vec![];

        while let Some(combination) = combination_length_gen.next() {
            combinations.push(combination);
        }

        assert_eq!(combinations.len(), 21);
    }
}
