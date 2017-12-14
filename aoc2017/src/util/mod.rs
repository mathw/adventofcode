pub mod asmillis;
pub mod pairs;
pub mod knothash;

pub use self::knothash::knot_hash;

use std::time::Instant;
use self::asmillis::AsMillis;

pub fn char_to_digit(c: char) -> Option<u8> {
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

pub fn repeatedly<F, R>(count: usize, what: F) -> R
where
    F: Fn() -> R,
{
    if count < 1 {
        panic!();
    }

    let result = what();

    if count > 1 {
        for _ in 1..count {
            what();
        }
    }

    result
}

#[allow(unused)]
pub fn timed<W, R>(work: W) -> (R, u64)
where
    W: Fn() -> R,
{
    let timer = Instant::now();
    (work(), timer.elapsed().as_millis())
}

pub fn timed_repeatedly<W, R>(count: usize, work: W) -> (R, u64)
where
    W: Fn() -> R,
{
    let timer = Instant::now();
    (repeatedly(count, || work()), timer.elapsed().as_millis())
}
