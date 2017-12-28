extern crate assembly;
extern crate util;

use util::timed;

fn main() {
    let input = include_str!("input.txt");

    let (result, time) = timed(|| part1(input));
    println!("[{}ms] mul was called {} times", time, result);

    let (result, time) = timed(|| part2());
    println!("[{}ms] value of h at end was {}", time, result);
}

fn part1(input: &str) -> usize {
    // abstracting all of this was a huge waste of time!
    assembly::run_for_day_23_part_one(input).unwrap()
}

fn part2() -> i64 {
    // this is almost just a completely different problem anyway
    let mut b: i64 = 107_900;
    let mut f: i64;
    let c = 124_900i64;
    let mut h: i64 = 0;

    loop {
        f = 1;

        'outer: for d in 2..b {
            for e in 2..(b / d) + 1 {
                if d * e == b {
                    f = 0;
                    break 'outer;
                }
            }
        }

        if f == 0 {
            h = h + 1;
        }

        if b == c {
            return h;
        }

        b = b + 17;
    }
}
