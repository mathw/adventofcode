use std::ops::Sub;
use std::str::FromStr;
use std::time::Instant;
use util::asmillis::AsMillis;
use util;
use util::pairs::IntoPairs;

pub fn go(count: usize) {
    let input = include_str!("input.txt");
    let input = parse_input(input);

    let timer1 = Instant::now();

    let part1 = util::repeatedly(count, || checksum(&input));

    println!("[{}ms] checksum is {}", timer1.elapsed().as_millis(), part1);

    let timer2 = Instant::now();

    let part2 = util::repeatedly(count, || checksum2(&input));

    println!("[{}ms] checksum2 is {}",
             timer2.elapsed().as_millis(),
             part2);
}

fn parse_row(line: &str) -> Vec<u32> {
    line.split_whitespace().filter_map(|x| u32::from_str(x).ok()).collect()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(parse_row).collect()
}

fn row_checksum<T: Ord + Sub<Output = T> + Clone>(row: &[T]) -> Option<T> {
    match (row.iter().min(), row.iter().max()) {
        (Some(min), Some(max)) => Some(max.clone() - min.clone()),
        _ => None,
    }
}

fn checksum(rows: &[Vec<u32>]) -> u32 {
    rows.iter().map(|x| row_checksum(x)).map(|x| x.unwrap_or(0)).sum()
}

fn row_checksum2(row: &Vec<u32>) -> u32 {
    row.iter()
        .pairs()
        .filter_map(|(&a, &b)| if a % b == 0 {
            Some(a / b)
        } else if b % a == 0 {
            Some(b / a)
        } else {
            None
        })
        .sum()
}

fn checksum2(rows: &[Vec<u32>]) -> u32 {
    rows.iter().map(|x| row_checksum2(x)).sum()
}

#[test]
fn test_parse_row() {
    assert_eq!(parse_row(""), vec![], "Empty row should be an empty vec");
    assert_eq!(parse_row("45"),
               vec![45],
               "Single number should produce that number");
    assert_eq!(parse_row("5 89    9         342"),
               vec![5, 89, 9, 342],
               "Row with varied whitespace shouldn't matter");
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("5 7\n8 9"), vec![vec![5, 7], vec![8, 9]]);
}

#[test]
fn test_row_checksum() {
    assert_eq!(row_checksum(&[1, 2]), Some(1));
    assert_eq!(row_checksum(&[67, 7, 8]), Some(60));
    assert_eq!(row_checksum(&Vec::<u32>::new()), None);
}

#[test]
fn test_checksum() {
    assert_eq!(checksum(&[vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]),
               18,
               "Checksum from sample data should be correct");
}

#[test]
fn test_checksum2() {
    assert_eq!(checksum2(&[vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]]),
               9,
               "Checksum for part 2 sample should be correct");
}

#[test]
fn test_row_checksum2() {
    assert_eq!(row_checksum2(&vec![5, 9, 2, 8]), 4);
    assert_eq!(row_checksum2(&vec![9, 4, 7, 3]), 3);
    assert_eq!(row_checksum2(&vec![3, 8, 6, 5]), 2);
}
