use crate::dayerror::DayError;

const KEY_1: u64 = 14205034;
const KEY_2: u64 = 18047856;

pub fn part1() -> Result<String, DayError> {
    let (loop1, loop2, key1, key2) = run_part1(KEY_1, KEY_2);
    Ok(format!(
        "Loop sizes {} and {}, keys {} and {}",
        loop1, loop2, key1, key2
    ))
}

pub fn part2() -> Result<String, DayError> {
    Ok("Doh!".into())
}

fn run_part1(key1: u64, key2: u64) -> (u64, u64, u64, u64) {
    let loop_size_1 = determine_loop_size(7, key1);
    let loop_size_2 = determine_loop_size(7, key2);
    let encryption_key = transform(key1, loop_size_2);
    let encryption_key_2 = transform(key2, loop_size_1);
    (loop_size_1, loop_size_2, encryption_key, encryption_key_2)
}

fn crypto_step(value: u64, subject_number: u64) -> u64 {
    (value * subject_number) % 20201227
}

fn determine_loop_size(subject_number: u64, final_value: u64) -> u64 {
    let mut loop_size = 0;
    let mut value = 1;
    while value != final_value {
        value = crypto_step(value, subject_number);
        loop_size += 1;
    }
    loop_size
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = crypto_step(value, subject_number);
    }
    value
}

#[test]
fn test_loop_size() {
    assert_eq!(determine_loop_size(7, 5764801), 8);
    assert_eq!(determine_loop_size(7, 17807724), 11);
}

#[test]
fn test_transform() {
    assert_eq!(transform(17807724, 8), 14897079);
    assert_eq!(transform(5764801, 11), 14897079);
}

#[test]
fn test_part_1() {
    let (loop_size_1, loop_size_2, key_1, key_2) = run_part1(5764801, 17807724);
    assert_eq!(loop_size_1, 8, "First key loop size");
    assert_eq!(loop_size_2, 11, "Second key loop size");
    assert_eq!(key_1, 14897079, "First key");
    assert_eq!(key_2, 14897079, "Second key");
}
