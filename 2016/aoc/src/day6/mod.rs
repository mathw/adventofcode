use std::collections::BTreeMap;

/// count letters in each column in the input lines
/// the input should be producing an iterator of lines
/// the output is a string of the most frequent letter in each column and a string of the least frequent
fn most_and_least_frequent_letter_in_columns<'a, I>(input: I) -> (String, String)
    where I: IntoIterator<Item = &'a str>
{
    let mut maps = Vec::new();

    let mut input_iterator = input.into_iter().peekable();

    // how long is the first line? We assume all lines are the same length
    let first_line_count = input_iterator.peek().unwrap().len();

    // create a new map for each column
    for _ in 0..first_line_count {
        maps.push(BTreeMap::new());
    }

    for line in input_iterator {
        for (i, c) in line.chars().enumerate() {
            if i < first_line_count {
                let e = maps[i].entry(c).or_insert(0);
                *e += 1;
            }
        }
    }

    (maps.iter().map(|m| get_most_common_letter(m)).collect(),
     maps.iter().map(|m| get_least_common_letter(m)).collect())
}

fn get_most_common_letter(m: &BTreeMap<char, u32>) -> char {
    let mut most_common_seen = ('a', 0);

    for (c, count) in m {
        if *count > most_common_seen.1 {
            most_common_seen = (*c, *count);
        }
    }

    most_common_seen.0
}

fn get_least_common_letter(m: &BTreeMap<char, u32>) -> char {
    let mut least_common_seen = ('a', u32::max_value());

    for (c, count) in m {
        if *count < least_common_seen.1 {
            least_common_seen = (*c, *count);
        }
    }

    least_common_seen.0
}

pub fn do_day6(input: &str) {
    let (most_frequent, least_frequent) = most_and_least_frequent_letter_in_columns(input.lines());

    println!("Most common message is {}", most_frequent);
    println!("Least common message is {}", least_frequent);
}

#[test]
fn test_get_most_common_letter() {
    let mut map = BTreeMap::new();
    map.insert('a', 2);
    map.insert('b', 1);
    map.insert('d', 5);
    map.insert('x', 55);

    assert_eq!(get_most_common_letter(&map), 'x');
}

#[test]
fn test_get_least_common_letter() {
    let mut map = BTreeMap::new();
    map.insert('a', 2);
    map.insert('b', 1);
    map.insert('d', 5);
    map.insert('x', 55);

    assert_eq!(get_least_common_letter(&map), 'b');
}
