use std::collections::VecDeque;

pub struct Generator {
    target_len: usize,
    min_word_len: usize,
    max_word_len: usize,
    state: Vec<usize>,
}

impl Generator {
    pub fn new(target_len: usize, min_word_len: usize, max_word_len: usize, max_words: usize) -> Self {
        let state = vec![min_word_len; max_words];
        Generator {
            target_len,
            min_word_len,
            max_word_len,
            state
        }
    }

    fn is_valid(&self) -> bool {
        if self.state.iter().sum::<usize>() != self.target_len {
            return false;
        }

        for value in self.state.iter() {
            if value > &self.max_word_len || value < &self.min_word_len {
                return false;
            }
        }
        true
    }

    fn increment(&mut self) -> Result<(), String> {
        for value in self.state.iter_mut() {
            if *value < self.max_word_len {
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
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.increment() == Ok(()) {
            if self.state.iter().sum::<usize>() == self.target_len && self.is_valid() {
                return Some(self.state.clone());
            }
        }
        None
    }
}

pub struct CombinationGenerator {
    generators: VecDeque<Generator>,
    seen_combinations: Vec<Vec<usize>>,
}

impl CombinationGenerator {
    pub fn new(target_len: usize, min_word_len: usize, max_word_len: usize, max_words: usize) -> Self {
        let mut generators = VecDeque::new();
        for max_words in 1..max_words + 1 {
            if max_words * max_word_len >= target_len {
                generators.push_back(
                    Generator::new(target_len, min_word_len, max_word_len, max_words)
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
        let mut generator = Generator::new(10, 2, 6, 3);
        generator.state = state;

        let _ = generator.increment();

        assert_eq!(expected, generator.state);
    }

    #[test_case(vec![1, 1, 1], false)]
    #[test_case(vec![5, 3, 2], true)]
    #[test_case(vec![4, 4, 2], true)]
    #[test_case(vec![1, 4, 2], false)]
    fn is_valid_returns_expected(state: Vec<usize>, expected: bool) {
        let mut generator = Generator::new(10, 2, 6, 3);
        generator.state = state;

        assert_eq!(generator.is_valid(), expected);
    }

    #[test]
    fn get_next_combination_returns_valid_combination_lengths() {
        let mut generator = Generator::new(21, 3, 10, 3);

        assert_eq!(generator.next().unwrap(), vec![10, 8, 3]);
        assert_eq!(generator.next().unwrap(), vec![9, 9, 3]);
    }

    #[test]
    fn combination_generator_calls_generators_in_turns() {
        let mut combination_generator = CombinationGenerator::new(8, 2, 6, 3);

        assert_eq!(combination_generator.next().unwrap().len(), 2);
        assert_eq!(combination_generator.next().unwrap().len(), 3);
        assert_eq!(combination_generator.next().unwrap().len(), 2);
    }

    #[test]
    fn combination_generator_returns_expected_number_of_combinations() {
        let mut combination_length_gen = CombinationGenerator::new(18, 3, 10, 4);
        let mut combinations = vec![];

        while let Some(combination) = combination_length_gen.next() {
            combinations.push(combination);
        }

        assert_eq!(combinations.len(), 21);
    }
}
