use day::Day;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Day2 {
    input: &'static str,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            input: include_str!("input.txt"),
        }
    }
}

impl Day for Day2 {
    fn part1(&mut self, sender: &Sender<String>) {
        let codes = parse_input(self.input);
        let checksum = checksum(codes);

        sender
            .send(format!("The checksum is {}", checksum))
            .unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let codes = parse_input(self.input).collect::<Vec<_>>();
        match find_close_codes(&codes) {
            Some(common_letters) => sender
                .send(format!("Common letters are {}", common_letters))
                .unwrap(),
            None => sender.send(format!("No close codes found")).unwrap(),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

fn has_letter_appearing_exactly_two_or_three_times(code: &str) -> (bool, bool) {
    let mut map = HashMap::new();
    for letter in code.chars() {
        let entry = map.entry(letter).or_insert(0);
        *entry += 1;
    }

    let twice = map.values().any(|&v| v == 2);
    let thrice = map.values().any(|&v| v == 3);

    (twice, thrice)
}

fn checksum<'a, Item: AsRef<str>, I: Iterator<Item = Item>>(codes: I) -> u32 {
    let mut twice = 0;
    let mut thrice = 0;

    for code in codes {
        let (tw, thr) = has_letter_appearing_exactly_two_or_three_times(code.as_ref());
        if tw {
            twice += 1;
        }
        if thr {
            thrice += 1;
        }
    }

    twice * thrice
}

fn has_one_letter_different(first: &str, second: &str) -> bool {
    if first.len() != second.len() {
        return false;
    }

    let mut different = 0;

    for (a, b) in first.chars().zip(second.chars()) {
        if a != b {
            different += 1;
        }
        if different > 1 {
            return false;
        }
    }

    different == 1
}

fn common_letters(first: &str, second: &str) -> Vec<char> {
    let mut common = Vec::new();

    for (a, b) in first.chars().zip(second.chars()) {
        if a == b {
            common.push(a);
        }
    }

    common
}

fn find_close_codes(codes: &Vec<&str>) -> Option<String> {
    for outer in codes {
        for inner in codes {
            if has_one_letter_different(outer, inner) {
                return Some(common_letters(outer, inner).iter().collect());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_appears_twice() {
        let (twice, thrice) = has_letter_appearing_exactly_two_or_three_times("aa");
        assert_eq!(twice, true, "Letter a appears exactly twice");
        assert_eq!(thrice, false, "No letter appears exactly thrice");
    }

    #[test]
    fn letter_appears_thrice() {
        let (twice, thrice) = has_letter_appearing_exactly_two_or_three_times("aaa");
        assert_eq!(thrice, true, "Letter a appears exactly thrice");
        assert_eq!(twice, false, "No letter appears exactly twice");
    }

    #[test]
    fn letter_appears_thrice_and_twice() {
        let (twice, thrice) = has_letter_appearing_exactly_two_or_three_times("aaabb");
        assert_eq!(thrice, true, "Letter a appears exactly thrice");
        assert_eq!(twice, true, "Letter b appears exactly twice");
    }

    #[test]
    fn no_letter_twice_or_thrice() {
        let (twice, thrice) = has_letter_appearing_exactly_two_or_three_times("abcdeffff");
        assert_eq!(twice, false, "No letter appears exactly twice");
        assert_eq!(thrice, false, "No letter appears exactly thrice");
    }

    #[test]
    fn checksum_example() {
        let v = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        let cs = checksum(v.iter());
        assert_eq!(cs, 12);
    }

    #[test]
    fn no_letters_different() {
        assert_eq!(has_one_letter_different("a", "a"), false)
    }

    #[test]
    fn one_letter_different() {
        assert_eq!(has_one_letter_different("aaa", "aba"), true);
    }

    #[test]
    fn two_letters_different() {
        assert_eq!(has_one_letter_different("aaa", "bba"), false);
    }

    #[test]
    fn common_letters() {
        assert_eq!(super::common_letters("a", "a"), vec!['a']);
        assert_eq!(super::common_letters("aa", "aa"), vec!['a', 'a']);
        assert_eq!(super::common_letters("abc", "aac"), vec!['a', 'c']);
    }

    #[test]
    fn close_codes_example() {
        assert_eq!(
            find_close_codes(&vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"
            ]),
            Some("fgij".to_owned())
        );
    }
}
