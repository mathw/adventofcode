use md5;
use rustc_serialize::hex::ToHex;
use std::string::ToString;
use std::collections::HashMap;

pub fn do_day14() {
    let mut space = SearchSpace::new("jlmsuwbz", false);
    let indexes = space.get_valid_keys(64);

    println!("Index of 64th key is {}", indexes[63]);

    let mut stretched_space = SearchSpace::new("jlmsuwbz", true);
    let stretched_indexes = stretched_space.get_valid_keys(64);

    println!("Index of 64th stretched key is {}", stretched_indexes[63]);
}

fn md5sum(input: &str, stretched: bool) -> String {
    let base = md5::compute(input.as_bytes()).to_hex();
    if stretched {
        stretch_hash(&base, 2016)
    } else {
        base
    }
}

fn stretch_hash(input: &str, count: usize) -> String {
    if count == 1 {
        md5::compute(input.as_bytes()).to_hex()
    } else {
        md5::compute(stretch_hash(input, count - 1).as_bytes()).to_hex()
    }
}

fn salted_hash(index: u64, salt: &String, stretched: bool) -> String {
    md5sum(&(salt.clone() + &index.to_string()), stretched)
}

/// Does this byte slice contain a triple-repeated character
fn contains_triple(haystack: &[u8]) -> Option<char> {
    if haystack.len() < 3 {
        return None;
    }
    for i in 0..haystack.len() - 2 {
        let thischar = haystack[i];
        if haystack[i + 1] == thischar && haystack[i + 2] == thischar {
            return Some(thischar as char);
        }
    }

    None
}

fn contains_pentuple_of(haystack: &[u8], c: char) -> bool {
    if haystack.len() < 5 {
        return false;
    }
    let cc = c as u8;
    for i in 0..haystack.len() - 4 {
        if haystack[i] == cc && haystack[i + 1] == cc && haystack[i + 2] == cc &&
           haystack[i + 3] == cc && haystack[i + 4] == cc {
            return true;
        }
    }

    false
}

struct SearchSpace {
    generated: HashMap<u64, String>,
    salt: String,
    stretch: bool,
}

impl SearchSpace {
    fn new(salt: &str, stretch: bool) -> SearchSpace {
        SearchSpace {
            generated: HashMap::new(),
            salt: salt.to_owned(),
            stretch: stretch,
        }
    }

    fn generate(&mut self, index: u64) -> String {
        if let Some(existing) = self.generated.get(&index) {
            return existing.clone();
        }

        let new = salted_hash(index, &self.salt, self.stretch);
        self.generated.insert(index, new.clone());
        new
    }

    fn first_triple_from(&mut self, start: u64) -> (u64, char) {
        let hash = self.generate(start);
        if let Some(c) = contains_triple(hash.as_bytes()) {
            (start, c)
        } else {
            self.first_triple_from(start + 1)
        }
    }

    fn first_pentuple_of_from_to(&mut self, of: char, start: u64, limit: u64) -> Option<u64> {
        let hash = self.generate(start);
        if contains_pentuple_of(hash.as_bytes(), of) {
            Some(start)
        } else if start + 1 == limit {
            None
        } else {
            self.first_pentuple_of_from_to(of, start + 1, limit)
        }
    }

    fn find_first_key_from(&mut self, start: u64) -> u64 {
        let (idx, c) = self.first_triple_from(start);
        if let Some(_) = self.first_pentuple_of_from_to(c, idx + 1, idx + 1001) {
            idx
        } else {
            self.find_first_key_from(idx + 1)
        }
    }

    /// return the indexes of valid keys found, up to count keys
    fn get_valid_keys(&mut self, count: u64) -> Vec<u64> {
        let mut v = Vec::new();
        let mut current_index = 0;

        loop {
            if v.len() as u64 == count {
                return v;
            }

            let key_index = self.find_first_key_from(current_index);
            v.push(key_index);
            current_index = key_index + 1;
        }
    }
}

#[test]
fn test_example_stuff() {
    let salt = "abc".to_owned();
    // we know index 18 will produce a hash containing "cc38887a5"
    assert!(salted_hash(18, &salt, false).find("cc38887a5").is_some());

    let mut space = SearchSpace::new("abc", false);
    // we also know that's the first triple
    let (first_index, first_char) = space.first_triple_from(0);
    assert_eq!(first_index, 18);
    assert_eq!(first_char, '8');

    // and that the next 1000 don't contain five 8
    assert!(space.first_pentuple_of_from_to('8', 19, 1019).is_none());

    // the next triple is 39
    let (second_index, second_char) = space.first_triple_from(19);
    assert_eq!(second_index, 39);
    assert_eq!(second_char, 'e');

    // and we know 816 has a pentuple e
    let first_pentuple_idx = space.first_pentuple_of_from_to('e', 40, 1040);
    assert_eq!(first_pentuple_idx, Some(816));

    assert_eq!(space.find_first_key_from(0), 39);
    assert_eq!(space.find_first_key_from(40), 92);

    let indexes = space.get_valid_keys(2);
    assert_eq!(indexes[0], 39);
    assert_eq!(indexes[1], 92);
}

#[test]
fn test_contains_triple() {
    assert_eq!(contains_triple(b"555"), Some('5'));
    assert_eq!(contains_triple(b"556"), None);
    assert_eq!(contains_triple(b"fried eggs are greeeeeat"), Some('e'));
}

#[test]
fn test_contains_pentuple_of() {
    assert_eq!(contains_pentuple_of(b"555", '5'), false);
    assert_eq!(contains_pentuple_of(b"556", '5'), false);
    assert_eq!(contains_pentuple_of(b"fried eggs are greeeeeat", 'e'), true);
}
