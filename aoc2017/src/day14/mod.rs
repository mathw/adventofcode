use util::knot_hash;
use util::timed;

pub fn go() {
    let input = "hwlqcszp";

    let (result, time) = timed(|| count_used_in_grid(input));
    println!("[{}ms] {} squares used in the grid", time, result);
}

fn count_used_in_grid(input: &str) -> usize {
    (0..128)
        .map(|r| row(input, r).iter().filter(|&&x| x).count())
        .sum()
}

fn row(input: &str, row: usize) -> Vec<bool> {
    let input = format!("{}-{}", input, row);
    hash_as_bits(&knot_hash(input.chars().map(|c| c as u8)))
}

fn hash_as_bits(hash: &str) -> Vec<bool> {
    hash.chars().flat_map(|c| char_to_bits(c)).collect()
}

fn char_to_bits(c: char) -> Vec<bool> {
    match c {
        '0' => vec![false, false, false, false],
        '1' => vec![false, false, false, true],
        '2' => vec![false, false, true, false],
        '3' => vec![false, false, true, true],
        '4' => vec![false, true, false, false],
        '5' => vec![false, true, false, true],
        '6' => vec![false, true, true, false],
        '7' => vec![false, true, true, true],
        '8' => vec![true, false, false, false],
        '9' => vec![true, false, false, true],
        'a' => vec![true, false, true, false],
        'b' => vec![true, false, true, true],
        'c' => vec![true, true, false, false],
        'd' => vec![true, true, false, true],
        'e' => vec![true, true, true, false],
        'f' => vec![true, true, true, true],
        _ => panic!("Hash value out of bounds {}", c),
    }
}
