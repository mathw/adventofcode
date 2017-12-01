pub fn go() {
    // parse
    let input = include_str!("input.txt");

    let digits = parse_digits(input.trim());
    if digits.len() == 0 {
        panic!("No input");
    }

    // part one
    let partone = sum_as_u32(&digits_matching_next(&digits));

    println!("The sum of all matching digits is {}", partone);

    // part two
    let parttwo = sum_as_u32(&items_matching_halfway_round(&digits));

    println!("The sum of all digits which match the digit halfway around the list is {}",
             parttwo);
}

fn sum_as_u32(items: &[u8]) -> u32 {
    items.iter().map(|&x| x as u32).sum()
}

fn parse_digits(input: &str) -> Vec<u8> {
    let digits = input.chars().map(char_to_digit);

    digits.map(|x| {
            x.expect(&format!("The input string contained something which was not a digit. It \
                               was {}",
                              input))
        })
        .collect::<Vec<_>>()
}

fn digits_matching_next<T: Eq + Clone>(digits: &[T]) -> Vec<T> {
    let mut matches_next = digits.windows(2)
        .filter(|window| window[0] == window[1])
        .map(|window| window[0].clone())
        .collect::<Vec<_>>();

    let last = digits[digits.len() - 1].clone();
    if digits[0] == last {
        matches_next.push(last);
    }

    matches_next
}

fn items_matching_halfway_round<T: Clone + Eq>(items: &[T]) -> Vec<T> {
    let mut matches = Vec::new();

    for (index, item) in items.iter().enumerate() {
        if get_item_halfway_round(items, index) == *item {
            matches.push(item.clone());
        }
    }

    matches
}

fn get_item_halfway_round<T: Clone>(source: &[T], index: usize) -> T {
    let steps = source.len() / 2;
    let index = (index + steps) % source.len();
    source[index].clone()
}

fn char_to_digit(c: char) -> Option<u8> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}


#[cfg(test)]
fn match_test_helper(digits: Vec<u8>, expected: Vec<u8>) {
    let matches = digits_matching_next(&digits);

    assert_eq!(matches, expected);
}

#[test]
fn test_digits_matching_next_none_match() {
    match_test_helper(vec![0, 1, 2, 3, 4], vec![]);
}

#[test]
fn test_digits_matching_next_all_match() {
    match_test_helper(vec![1, 1, 1, 1], vec![1, 1, 1, 1]);
}

#[test]
fn test_digits_matching_next_some_match() {
    match_test_helper(vec![2, 3, 4, 4, 2], vec![4, 2]);
}

#[test]
fn test_parse_digits() {
    assert_eq!(parse_digits(""), vec![]);
    assert_eq!(parse_digits("0"), vec![0]);
    assert_eq!(parse_digits("23288839"), vec![2, 3, 2, 8, 8, 8, 3, 9]);
}

#[test]
fn test_item_halfway_round() {
    assert_eq!(get_item_halfway_round(&[0, 1, 2, 3, 4, 5], 1), 4);
}