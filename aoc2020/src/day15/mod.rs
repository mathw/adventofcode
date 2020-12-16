use crate::dayerror::DayError;
use std::collections::HashMap;

pub fn part1() -> Result<String, DayError> {
    let result = run_memory_game(&[9, 6, 0, 10, 18, 2, 1], 2020);

    Ok(format!("The 2020th number spoken is {}", result))
}

pub fn part2() -> Result<String, DayError> {
    let result = run_memory_game(&[9, 6, 0, 10, 18, 2, 1], 30000000);

    Ok(format!("The thirty millionth number spoken is {}", result))
}

fn memory_game_turn(memory: &mut HashMap<u32, u32>, number: u32, turn_number: u32) -> u32 {
    if let Some(last_spoken) = memory.get(&number) {
        let difference = turn_number - last_spoken;
        memory.insert(number, turn_number);
        difference
    } else {
        memory.insert(number, turn_number);
        0
    }
}

fn run_memory_game(starting_numbers: &[u32], iterations: u32) -> u32 {
    let mut memory = HashMap::new();
    let mut turn = 0;
    let mut number_to_speak = 0;

    for number in starting_numbers {
        turn += 1;
        number_to_speak = memory_game_turn(&mut memory, *number, turn);
    }

    while turn < iterations - 1 {
        turn += 1;
        number_to_speak = memory_game_turn(&mut memory, number_to_speak, turn);
    }

    number_to_speak
}

#[test]
fn test_samples() {
    assert_eq!(run_memory_game(&[0, 3, 6], 2020), 436);
    assert_eq!(run_memory_game(&[1, 3, 2], 2020), 1);
    assert_eq!(run_memory_game(&[2, 1, 3], 2020), 10);
    assert_eq!(run_memory_game(&[1, 2, 3], 2020), 27);
    assert_eq!(run_memory_game(&[2, 3, 1], 2020), 78);
    assert_eq!(run_memory_game(&[3, 2, 1], 2020), 438);
    assert_eq!(run_memory_game(&[3, 1, 2], 2020), 1836);
}

// don't run this - too slow in debug mode
// #[test]
// fn test_big_samples() {
//     assert_eq!(run_memory_game(&[0, 3, 6], 30000000), 175594);
//     // assert_eq!(run_memory_game(&[1, 3, 2], 30000000), 2578);
//     // assert_eq!(run_memory_game(&[2, 1, 3], 30000000), 3544142);
//     // assert_eq!(run_memory_game(&[1, 2, 3], 30000000), 261214);
//     // assert_eq!(run_memory_game(&[2, 3, 1], 30000000), 6895259);
//     // assert_eq!(run_memory_game(&[3, 2, 1], 30000000), 18);
//     // assert_eq!(run_memory_game(&[3, 1, 2], 30000000), 362);
// }
