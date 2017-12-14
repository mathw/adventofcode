use std::str::FromStr;
use util::timed;
use util::knothash::{hash, knot_hash};

pub fn go() {
    let input = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113";

    let (result, time) = timed(|| {
        part1(
            input
                .split(',')
                .filter_map(|x| usize::from_str(x).ok())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    });

    println!("[{}ms] hash is {}", time, result);

    let (result, time) = timed(|| part2(input.chars().map(|c| c as u8)));

    println!("[{}ms] hash is {}", time, result);
}

fn part1(input: &[usize]) -> u16 {
    let start = (0..256).collect::<Vec<u16>>();

    hash(start.as_slice(), input)
}

fn part2<I>(input: I) -> String
where
    I: IntoIterator<Item = u8>,
{
    knot_hash(input)
}
