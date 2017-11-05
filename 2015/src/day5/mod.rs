use std::str;

pub fn run() {
    let input = include_str!("input.txt");

    let nice_strings = input.lines().filter(|l| string_is_nice(l)).count();

    println!("{} strings are nice", nice_strings);

    let nice_strings = input.lines().filter(|l| string_is_nice_2(l)).count();

    println!("Under the new rules, {} strings are nice", nice_strings);
}

fn string_is_nice_2(s: &str) -> bool {
    let b = s.as_bytes();

    let has_aba = b.windows(3).any(|trio| trio[0] == trio[2]);
    let has_pair = b.windows(2).enumerate().any(|(index, pair)| {
        s.rfind(str::from_utf8(pair).unwrap()).map(|i| i > index + 1).unwrap_or(false)
    });

    return has_aba && has_pair;
}

fn string_is_nice(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut previous_char = '!';
    let mut has_double = false;

    for c in s.chars() {
        match (previous_char, c) {
            ('a', 'b') => return false,
            ('c', 'd') => return false,
            ('p', 'q') => return false,
            ('x', 'y') => return false,
            _ => {}
        }

        if is_vowel(c) {
            vowel_count += 1;
        }

        if c == previous_char {
            has_double = true;
        }

        previous_char = c;
    }

    return vowel_count >= 3 && has_double;
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[test]
fn test_naughtiness() {
    fn assert_naughty(s: &str) {
        assert_eq!(string_is_nice(s), false, "{} should be naughty", s);
    }

    fn assert_nice(s: &str) {
        assert!(string_is_nice(s), "{} should be nice", s);
    }

    assert_nice("ugknbfddgicrmopn");
    assert_nice("aaa");
    assert_naughty("jchzalrnumimnmhp");
    assert_naughty("haegwjzuvuyypxyu");
    assert_naughty("dvszwmarrgswjxmb");
}

#[test]
fn test_naughtiness_2() {
    fn assert_naughty(s: &str) {
        assert_eq!(string_is_nice_2(s), false, "{} should be naughty", s);
    }

    fn assert_nice(s: &str) {
        assert!(string_is_nice_2(s), "{} should be nice", s);
    }

    assert_nice("qjhvhtzxzqqjkmpb");
    assert_nice("xxyxx");
    assert_nice("uurcxstgmgygtbstg");
    assert_nice("bbaaabb");
    assert_naughty("aaaodo");
    assert_naughty("uurcxstgmygtbstg");
    assert_naughty("ieodomkazucvgmuy");
}

#[test]
fn test_is_vowel() {
    fn assert_vowel(c: char) {
        assert!(is_vowel(c), "{} should be a vowel", c);
    }

    fn assert_not_vowel(c: char) {
        assert!(!is_vowel(c), "{} should not be a vowel", c);
    }

    assert_vowel('a');
    assert_vowel('e');
    assert_vowel('i');
    assert_vowel('o');
    assert_vowel('u');
    assert_not_vowel('b');
    assert_not_vowel(' ');
    assert_not_vowel('!');
}