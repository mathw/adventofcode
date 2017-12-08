mod ast;
mod parser;
mod interpreter;

use self::parser::parse_program;
use self::interpreter::run;
use util::timed;

pub fn go() {
    let source_text = include_str!("input.txt");

    let (result, time) = timed(|| part1(source_text));
    println!("[{}ms] largest value in any register is {}", time, result);
}

fn part1(source: &str) -> i32 {
    let program = parse_program(source).unwrap();
    let registers = run(&program);
    registers.values().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    mod whole_program {
        use super::super::parser::parse_program;
        use super::super::interpreter::run;

        #[test]
        fn run_sample_program() {
            let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
            let parsed = parse_program(input);
            assert!(parsed.is_ok());
            let parsed = parsed.unwrap();
            let registers = run(&parsed);
            let largest = registers.values().max();
            assert_eq!(largest, Some(&1));
        }
    }
}