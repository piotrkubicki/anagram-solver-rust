use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Error};
use std::fs::File;

pub struct Dictionary {
    words: HashMap<usize, HashSet<String>>
}

impl Dictionary {
    pub fn new(file_path: &str, min_word_len: usize, max_word_len: usize, allowed_chars: &Vec<char>) -> Result<Self, Error> {
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let dictionary = Dictionary::map(&mut reader, min_word_len, max_word_len, allowed_chars);
        Ok(Dictionary {
            words: dictionary,
        })
    }

    pub fn get(&self, key: &usize) -> Option<HashSet<String>> {
        match self.words.get(key) {
            Some(set) => Some(set.clone()),
            _ => None
        }
    }

    fn map<T: BufRead>(reader: &mut T, min_word_len: usize, max_word_len: usize, allowed_chars: &Vec<char>) -> HashMap<usize, HashSet<String>> {
        let mut dictionary: HashMap<usize, HashSet<String>> = HashMap::new();

        for line in reader.lines() {
            if let Ok(mut word) = line {
                if Self::is_valid(min_word_len, max_word_len, allowed_chars, &word) {
                    let word = Dictionary::clean(&mut word).to_string();
                    let word_len = word.len();
                    if dictionary.contains_key(&word_len) {
                        if let Some(set) = dictionary.get_mut(&word_len) {
                            set.insert(word);
                        }
                    }
                    else {
                        dictionary.insert(word_len, HashSet::from([word]));
                    }
                }
            }
        }
        dictionary
    }

    fn is_valid(min_word_len: usize, max_word_len: usize, allowed_chars: &Vec<char>, word: &str) -> bool {
        let excluded_chars = "'";
        if word.len() < min_word_len || word.len() > max_word_len { return false };
        if word.chars().any(char::is_numeric) { return false }
        if excluded_chars.chars().map(|x| word.contains(x)).collect::<Vec<bool>>().contains(&true) { return false };

        let mut letter_count: HashMap<char, usize> = HashMap::new();
        for c in word.chars() {
            *letter_count.entry(c).or_insert(1) += 1;
        }
        let mut allowed_chars_count: HashMap<char, usize> = HashMap::new();
        for c in allowed_chars {
            *allowed_chars_count.entry(*c).or_insert(1) += 1;
        }
        for (k, v) in letter_count {
            if let Some(count) = allowed_chars_count.get(&k) {
                if v > *count {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn clean(word: &mut str) -> &str {
       word.trim_end_matches(|x| char::is_alphabetic(x) == false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn map_returns_expected_dictionary() {
        let wordlist = "this\nis\njust\na\ntest\nlet\nsee\nhow\nit's\nworks\ntest!";
        let expected: HashMap<usize, HashSet<String>> = HashMap::from([
            (4, HashSet::from(["this".to_string(), "just".to_string(), "test".to_string()])),
            (5, HashSet::from(["works".to_string()])),
        ]);
        let allowed_chars = vec!['i', 't', 't', 'e', 's', 'h', 'j', 'u', 'w', 'r', 'o', 'k'];

        assert_eq!(Dictionary::map(&mut wordlist.as_bytes(), 4, 8, &allowed_chars), expected);
    }

    #[test_case("valid", "validdt", true; "valid word")]
    #[test_case("k", "k", false; "to short")]
    #[test_case("cat's", "catssk", false; "contains invalid char")]
    #[test_case("tolongworld", "toolongword", false; "is to long")]
    #[test_case("conta1n", "contain", false; "contain digit")]
    #[test_case("seveeen", "neeeves", true; "max length")]
    #[test_case("cat", "catr", true; "min length")]
    fn is_valid_returns_correct_bool(word: &str, allowed_chars: &str, expected: bool) {
        let allowed_chars = allowed_chars.chars().into_iter().collect::<Vec<char>>();
        assert_eq!(Dictionary::is_valid(3, 7, &allowed_chars, word), expected);
    }

    #[test_case("word!", "word")]
    #[test_case("work34", "work")]
    #[test_case("worst;", "worst")]
    #[test_case("mors\n", "mors")]
    #[test_case("something?", "something")]
    fn clean_returns_word_with_removed_non_alphabetic_chars(word: &str, expected: &str) {
        assert_eq!(Dictionary::clean(&mut word.to_string()), expected);
    }
}
