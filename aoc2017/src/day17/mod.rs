use util::timed;

pub fn go() {
    let input = 312;

    let (result, time) = timed(|| part1(input));

    println!("[{}ms] {}", time, result);

    let (result, time) = timed(|| part2(input));

    println!("[{}ms] {}", time, result);
}

fn part1(steps: usize) -> u16 {
    let mut buffer = CircularBuffer::new(0u16, 2018);

    for n in 1..2018 {
        buffer.step_insert(steps, n);
    }

    buffer.buffer[(buffer.position + 1) % buffer.buffer.len()]
}

fn part2(steps: usize) -> u32 {
    let mut buffer = CircularBuffer::new(0u32, 50000001);

    for n in 1..50000000 {
        if n % 100000 == 0 {
            println!("{}", n);
        }
        buffer.step_insert_only_after_zero(steps, n);
    }

    buffer.buffer[1]
}

struct CircularBuffer<T> {
    buffer: Vec<T>,
    position: usize,
}

impl<T> CircularBuffer<T> {
    fn new(initial_value: T, desired_capacity: usize) -> CircularBuffer<T> {
        let mut v = Vec::with_capacity(desired_capacity);
        v.push(initial_value);
        CircularBuffer {
            buffer: v,
            position: 0,
        }
    }

    fn step_insert(&mut self, steps: usize, value: T) {
        self.position = (self.position + steps) % self.buffer.len();
        self.buffer.insert(self.position + 1, value);
        self.position += 1;
    }

    /// Optimised `step_insert` which only actually inserts something if it's at position zero
    /// which is the only one we're interested in for part two of the problem
    fn step_insert_only_after_zero(&mut self, steps: usize, value: T) {
        self.position = (self.position + steps) % self.buffer.len();
        if self.position == 0 {
            self.buffer.insert(self.position + 1, value);
        } else {
            // just put it on the end
            self.buffer.push(value);
        }
        self.position += 1;
    }
}
