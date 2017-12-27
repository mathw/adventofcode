pub mod asmillis;

use std::time::Instant;
use self::asmillis::AsMillis;

pub fn timed<W, R>(work: W) -> (R, u64)
where
    W: Fn() -> R,
{
    let timer = Instant::now();
    (work(), timer.elapsed().as_millis())
}
