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

    fn part2(&mut self, sender: &Sender<String>) {}
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
}
