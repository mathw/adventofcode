use std::collections::HashSet;
use util::timed_repeatedly;

pub fn go(reps: usize) {
    let input = include_str!("input.txt");

    let (result, time) = timed_repeatedly(reps, || how_many_passphrases_are_valid(input));

    println!("[{}ms] {} passphrases are valid", time, result);

    let (result, time) = timed_repeatedly(reps, || how_many_passphrases_are_valid_2(input));

    println!("[{}ms] {} passphrases are valid under the new rules",
             time,
             result);
}

fn how_many_passphrases_are_valid(passphrases: &str) -> usize {
    passphrases.lines().filter(|passphrase| is_valid_passphrase(passphrase)).count()
}

fn is_valid_passphrase(passphrase: &str) -> bool {
    !has_duplicates(passphrase.split_whitespace())
}

fn has_duplicates<'a, Words>(words: Words) -> bool
    where Words: IntoIterator<Item = &'a str>
{
    let mut map = HashSet::new();

    for word in words {
        if map.contains(word) {
            return true;
        }

        map.insert(word);
    }

    return false;
}

fn normalise_word(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.sort();
    chars.iter().collect()
}

fn has_anagram_words<'w, Words>(words: Words) -> bool
    where Words: IntoIterator<Item = &'w str>
{
    let mut normalised_words = HashSet::new();

    for word in words {
        let norm = normalise_word(word);
        if normalised_words.contains(&norm) {
            return true;
        }
        normalised_words.insert(norm);
    }

    return false;
}

fn is_valid_passphrase_2(passphrase: &str) -> bool {
    !has_anagram_words(passphrase.split_whitespace())
}

fn how_many_passphrases_are_valid_2(input: &str) -> usize {
    input.lines().filter(|passphrase| is_valid_passphrase_2(passphrase)).count()
}


#[cfg(test)]
mod tests {
    mod has_duplicates {
        use super::super::*;

        #[test]
        fn can_find_duplicates() {
            assert_eq!(has_duplicates(vec!["a", "b", "a"]), true);
        }

        #[test]
        fn can_ignore_non_duplicates() {
            assert_eq!(has_duplicates(vec!["a", "b", "c"]), false);
            assert_eq!(has_duplicates(vec!["a", "b", "aa"]), false);
        }

        #[test]
        fn can_handle_empty_words() {
            assert_eq!(has_duplicates(vec![]), false);
        }
    }

    mod normalise_word {
        use super::super::*;

        #[test]
        fn can_normalise_duplicate_letters() {
            assert_eq!(normalise_word("aaaa"), "aaaa".to_owned());
        }

        #[test]
        fn can_normalise_random_nonsense() {
            assert_eq!(normalise_word("iaoowe"), "aeioow".to_owned());
        }
    }

    mod has_anagram_words {
        use super::super::*;

        #[test]
        fn can_spot_anagrams() {
            assert_eq!(has_anagram_words("oiii ioii iioi iiio".split_whitespace()),
                       true);
        }

        #[test]
        fn can_ignore_non_anagrams() {
            assert_eq!(has_anagram_words("a b c d e".split_whitespace()), false);
        }
    }
}