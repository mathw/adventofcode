use util::timed;

static FACTOR_A: u64 = 16807;
static FACTOR_B: u64 = 48271;

static INPUT_A: u64 = 883;
static INPUT_B: u64 = 879;

static DIVISOR_A: u64 = 4;
static DIVISOR_B: u64 = 8;

pub fn go() {
    let (result, time) = timed(|| part1());

    println!("[{}ms] {} pairs", time, result);

    let (result, time) = timed(|| part2());

    println!("[{}ms] {} pairs", time, result);
}

fn part1() -> usize {
    matches_in_pairs(
        Generator::new(FACTOR_A, INPUT_A),
        Generator::new(FACTOR_B, INPUT_B),
        40000000,
    )
}

fn part2() -> usize {
    matches_in_pairs(
        Generator::new(FACTOR_A, INPUT_A).filter(|x| x % DIVISOR_A == 0),
        Generator::new(FACTOR_B, INPUT_B).filter(|x| x % DIVISOR_B == 0),
        5000000,
    )
}

struct Generator {
    factor: u64,
    last: u64,
}

impl Generator {
    pub fn new(factor: u64, seed: u64) -> Generator {
        Generator {
            factor: factor,
            last: seed,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.last = (self.last * self.factor) % 2147483647;
        Some(self.last)
    }
}

fn lowest_16_bits(num: u64) -> u16 {
    (num & 0xffff) as u16
}

fn matches_in_pairs<IteratorA, IteratorB>(a: IteratorA, b: IteratorB, count: usize) -> usize
where
    IteratorA: Iterator<Item = u64>,
    IteratorB: Iterator<Item = u64>,
{
    a.zip(b)
        .take(count)
        .filter(|&(x, y)| lowest_16_bits(x) == lowest_16_bits(y))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_a() {
        let generator = Generator::new(FACTOR_A, 65);
        let generated = generator.take(5).collect::<Vec<u64>>();
        assert_eq!(
            generated,
            vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]
        );
    }

    #[test]
    fn test_generator_b() {
        let generator = Generator::new(FACTOR_B, 8921);
        let generated = generator.take(5).collect::<Vec<u64>>();
        assert_eq!(
            generated,
            vec![430625591, 1233683848, 1431495498, 137874439, 285222916]
        );
    }

    #[test]
    fn test_lowest_16() {
        let num = 0b00001110101000101110001101001010;
        assert_eq!(lowest_16_bits(num), 0b1110001101001010);
    }

    #[test]
    fn test_sample() {
        assert_eq!(
            matches_in_pairs(
                Generator::new(FACTOR_A, 65),
                Generator::new(FACTOR_B, 8921),
                40000000
            ),
            588
        );
    }
}
