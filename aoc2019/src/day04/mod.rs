use itertools::Itertools;

const LOWERBOUND: usize = 387638;
const UPPERBOUND: usize = 919123;

pub fn run() -> Result<(), String> {
    let possibles = all_possible_passwords(LOWERBOUND, UPPERBOUND);
    println!("Part 1: There are {} possible passwords", possibles);
    let possibles = all_possible_passwords2(LOWERBOUND, UPPERBOUND);
    println!("Part 2: There are {} possible passwords", possibles);
    Ok(())
}

fn digits_only_increase(digits: &Vec<u8>) -> bool {
    let mut sorted = digits.clone();
    sorted.sort();
    sorted == *digits
}

fn has_two_equal_next_to(digits: &Vec<u8>) -> bool {
    digits.iter().tuple_windows().any(|(a, b)| *a == *b)
}

fn has_two_equal_next_to_not_part_of_larger_group(digits: &Vec<u8>) -> bool {
    let has_mid = digits
        .iter()
        .tuple_windows()
        .any(|(a, b, c, d)| *b == *c && *a != *b && *c != *d);
    let has_start = digits[0] == digits[1] && digits[1] != digits[2];
    let has_end = digits[4] == digits[5] && digits[3] != digits[4];
    has_mid || has_start || has_end
}

fn digitise(num: usize) -> Vec<u8> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn all_possible_passwords(lower: usize, upper: usize) -> usize {
    let mut count = 0;
    for candidate_num in lower..=upper {
        let digits = digitise(candidate_num);
        if digits_only_increase(&digits) && has_two_equal_next_to(&digits) {
            count += 1;
        }
    }

    count
}

fn all_possible_passwords2(lower: usize, upper: usize) -> usize {
    let mut count = 0;
    for candidate_num in lower..=upper {
        let digits = digitise(candidate_num);
        if is_valid_password2(&digits) {
            count += 1;
        }
    }

    count
}

fn is_valid_password2(digits: &Vec<u8>) -> bool {
    digits_only_increase(digits) && has_two_equal_next_to_not_part_of_larger_group(digits)
}

#[test]
fn test_digits_increase() {
    assert!(digits_only_increase(&vec![1, 2, 5, 9]));
    assert!(!digits_only_increase(&vec![9, 5, 6]));
    assert!(digits_only_increase(&vec![1, 1, 1, 1]));
}

#[test]
fn test_has_equal() {
    assert!(has_two_equal_next_to(&vec![1, 2, 1, 1]));
    assert!(!has_two_equal_next_to(&vec![1, 2, 1]));
}

#[test]
fn test_is_valid_password2() {
    assert!(is_valid_password2(&vec![1, 1, 2, 2, 3, 3]));
    assert!(!is_valid_password2(&vec![1, 2, 3, 4, 4, 4]));
    assert!(is_valid_password2(&vec![1, 1, 1, 1, 2, 2]));
}

#[test]
fn test_has_two_equal_next_to_not_part_of_larger_group() {
    assert!(has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 1, 2, 3, 4, 5
    ]));
    assert!(has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 8, 2, 3, 4, 4
    ]));
    assert!(has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 8, 2, 3, 3, 4
    ]));
    assert!(!has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 8, 2, 5, 3, 4
    ]));
    assert!(!has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 8, 2, 2, 2, 4
    ]));
    assert!(!has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 8, 4, 2, 2, 2
    ]));
    assert!(!has_two_equal_next_to_not_part_of_larger_group(&vec![
        1, 1, 1, 2, 3, 4
    ]));
}
