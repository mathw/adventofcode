use std::str::FromStr;
use util::timed_repeatedly;

pub fn go(count: usize) {
    let input = parse_input(include_str!("input.txt"));

    let (result, time) = timed_repeatedly(count, || part1(&input));
    println!("[{}ms] Part One: {} steps to escape", time, result);

    let (result, time) = timed_repeatedly(count, || part2(&input));
    println!("[{}ms] Part Two: {} steps to escape", time, result);
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.trim()).filter_map(|line| i32::from_str(line).ok()).collect()
}

fn part1(instructions: &[i32]) -> usize {
    let mut interpreter = Interpreter::new(instructions, InterpreterMode::PartOne);
    match interpreter.run() {
        Ok(count) => count,
        Err(msg) => panic!(msg),
    }
}

fn part2(instructions: &[i32]) -> usize {
    let mut interpreter = Interpreter::new(instructions, InterpreterMode::PartTwo);
    match interpreter.run() {
        Ok(count) => count,
        Err(msg) => panic!(msg),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterpreterMode {
    PartOne,
    PartTwo,
}

struct Interpreter {
    instructions: Vec<i32>,
    current: usize,
    mode: InterpreterMode,
}

impl Interpreter {
    fn new(instructions: &[i32], mode: InterpreterMode) -> Interpreter {
        Interpreter {
            instructions: instructions.iter().cloned().collect(),
            current: 0,
            mode: mode,
        }
    }

    #[cfg(test)]
    fn print(&self) {
        let items = self.instructions
            .iter()
            .enumerate()
            .map(|(i, ins)| {
                if i == self.current {
                    format!("({})", ins)
                } else {
                    format!(" {} ", ins)
                }
            })
            .collect::<String>();
        println!("{}", items);
    }

    fn run(&mut self) -> Result<usize, &'static str> {
        let mut count = 0;

        #[cfg(test)]
        self.print();

        loop {
            count += 1;

            let jump = match self.instructions.get(self.current) {
                Some(j) => j.clone(),
                None => return Err("Unable to get current instruction"),
            };
            let new_current = self.current as isize + jump as isize;

            if new_current < 0 || new_current >= self.instructions.len() as isize {
                break;
            }

            if let Some(current_instruction) = self.instructions.get_mut(self.current) {
                *current_instruction = match self.mode {
                    InterpreterMode::PartOne => *current_instruction + 1,
                    InterpreterMode::PartTwo => {
                        if jump >= 3 {
                            *current_instruction - 1
                        } else {
                            *current_instruction + 1
                        }
                    }
                };
            } else {
                return Err("Unable to update current instruction as it was not found");
            }
            self.current = new_current as usize;

            #[cfg(test)]
            self.print();
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    mod interpreter {
        use super::super::*;

        #[test]
        fn test_interpret() {
            let mut interpreter = Interpreter::new(&[0, 3, 0, 1, -3], InterpreterMode::PartOne);
            let result = interpreter.run();
            assert_eq!(result, Ok(5));
        }

        #[test]
        fn test_interpret_parttwo() {
            let mut interpreter = Interpreter::new(&[0, 3, 0, 1, -3], InterpreterMode::PartTwo);
            let result = interpreter.run();
            assert_eq!(result, Ok(10));
        }
    }
}