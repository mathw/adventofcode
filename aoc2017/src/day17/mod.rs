use util::timed;

pub fn go() {
    let input = 312;

    let (result, time) = timed(|| part1(input));

    println!("[{}ms] {}", time, result);
}

fn part1(steps: usize) -> u16 {
    let mut buffer = CircularBuffer::new(0u16);

    for n in 1..2018 {
        buffer.step_insert(steps, n);
    }

    buffer.buffer[(buffer.position + 1) % buffer.buffer.len()]
}

struct CircularBuffer<T> {
    buffer: Vec<T>,
    position: usize,
}

impl<T> CircularBuffer<T> {
    fn new(initial_value: T) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![initial_value],
            position: 0,
        }
    }

    fn step_insert(&mut self, steps: usize, value: T) {
        self.position = (self.position + steps) % self.buffer.len();
        self.buffer.insert(self.position + 1, value);
        self.position += 1;
    }
}
