mod fsm;

use util::timed;

pub fn go() {
    let input = include_str!("input.txt");

    let ((score, garbage), time) = timed(|| part1(input));
    println!("[{}ms] score is {}, garbage count is {}",
             time,
             score,
             garbage);
}

fn part1(input: &str) -> (u32, u32) {
    fsm::run(input)
}
