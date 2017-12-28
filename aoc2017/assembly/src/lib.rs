use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::iter::Sum;
use std::str::FromStr;
use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RunMode {
    Day18PartOne,
    Day18PartTwo,
    Day23PartOne,
    Day23PartTwo,
}

pub fn run_for_day_23_part_one(source: &str) -> Result<usize, ()> {
    let program = parse_program(source)?;
    let (tx, rx) = channel();
    run_program(&program, RunMode::Day23PartOne, rx, tx, 0, false).map(|x| x as usize)
}

pub fn run_for_day_23_part_two(source: &str) -> Result<i64, ()> {
    let program = parse_program(source)?;
    let (tx, rx) = channel();
    run_program(&program, RunMode::Day23PartTwo, rx, tx, 0, true)
}

pub fn run_for_day_18_part2(source: &str) -> Result<i64, ()> {
    let program = parse_program(source)?;
    let program1 = program.clone();
    let program2 = program.clone();
    let (tx0, rx1) = channel();
    let (tx1, rx0) = channel();

    let prog0 = thread::spawn(move || {
        run_program(&program1, RunMode::Day18PartTwo, rx0, tx0, 0, false)
    });
    let prog1 = thread::spawn(move || {
        run_program(&program2, RunMode::Day18PartTwo, rx1, tx1, 1, false)
    });

    let _ = prog0.join();
    let prog1result = prog1.join();

    match prog1result {
        Ok(r) => r,
        Err(_) => Err(()),
    }
}

fn read_enter() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut buffer);
}

pub fn run_program(
    program: &Program,
    mode: RunMode,
    rx: Receiver<i64>,
    tx: Sender<i64>,
    prog_id: i64,
    step_mode: bool,
) -> Result<i64, ()> {
    let mut instructionp: isize = 0;
    let mut registers = HashMap::new();
    let mut last_sound = 0;
    let mut did_jump = false;

    if mode == RunMode::Day18PartTwo {
        set_register_value('p', prog_id, &mut registers);
    }

    if mode == RunMode::Day23PartTwo {
        set_register_value('a', 1, &mut registers);
    }

    let mut send_counter = 0;
    let mut multiply_counter = 0;

    while instructionp >= 0 && instructionp < program.0.len() as isize {
        if step_mode {
            println!("Registers: {:?}", registers);
            println!(
                "Execute [{}] {:?}?",
                instructionp,
                program.0[instructionp as usize]
            );
            //read_enter();
        }
        match program.0[instructionp as usize] {
            Instruction::Set(target, value) => set_register_value(
                target,
                get_parameter_value(value, &registers),
                &mut registers,
            ),
            Instruction::Add(target, value) => {
                let new_value =
                    get_register_value(target, &registers) + get_parameter_value(value, &registers);
                set_register_value(target, new_value, &mut registers);
            }
            Instruction::Sub(target, value) => {
                let new_value =
                    get_register_value(target, &registers) - get_parameter_value(value, &registers);
                set_register_value(target, new_value, &mut registers);
            }
            Instruction::Multiply(target, value) => {
                let new_value =
                    get_register_value(target, &registers) * get_parameter_value(value, &registers);
                set_register_value(target, new_value, &mut registers);
                multiply_counter += 1;
            }
            Instruction::Modulo(target, value) => {
                let new_value =
                    get_register_value(target, &registers) % get_parameter_value(value, &registers);
                set_register_value(target, new_value, &mut registers);
            }
            Instruction::Sound(frequency) => match mode {
                RunMode::Day18PartOne => {
                    last_sound = get_parameter_value(frequency, &registers);
                }
                RunMode::Day18PartTwo => {
                    tx.send(get_parameter_value(frequency, &registers))
                        .map_err(|_e| {
                            #[cfg(test)]
                            println!("Send error in {} {}", prog_id, _e);
                            ()
                        })?;
                    send_counter += 1;
                }
                _ => {}
            },
            Instruction::RecoverWhenNotZero(register) => match mode {
                RunMode::Day18PartOne => if get_register_value(register, &registers) != 0 {
                    set_register_value(register, last_sound, &mut registers);
                    return Ok(last_sound);
                },
                RunMode::Day18PartTwo => match rx.recv_timeout(Duration::from_secs(3)) {
                    Ok(val) => set_register_value(register, val, &mut registers),
                    Err(e) => match e {
                        RecvTimeoutError::Timeout => {
                            println!("Program {} timeout", prog_id);
                            return Ok(send_counter);
                        }
                        _ => return Err(()),
                    },
                },
                _ => {}
            },
            Instruction::JumpGreaterThanZero(check, offset) => {
                if get_parameter_value(check, &registers) > 0 {
                    instructionp += get_parameter_value(offset, &registers) as isize;
                    did_jump = true;
                }
            }
            Instruction::JumpNotZero(check, offset) => {
                if get_parameter_value(check, &registers) != 0 {
                    instructionp += get_parameter_value(offset, &registers) as isize;
                    did_jump = true;
                }
            }
        }

        if !did_jump {
            instructionp += 1;
        }

        did_jump = false;
    }

    match mode {
        RunMode::Day18PartOne => Err(()),
        RunMode::Day18PartTwo => Ok(send_counter),
        RunMode::Day23PartOne => Ok(multiply_counter),
        RunMode::Day23PartTwo => Ok(get_register_value('h', &registers)),
    }
}

fn get_parameter_value(p: Parameter, registers: &HashMap<RegisterName, i64>) -> i64 {
    match p {
        Parameter::Register(r) => get_register_value(r, registers),
        Parameter::Value(v) => v,
    }
}

fn get_register_value(r: RegisterName, registers: &HashMap<RegisterName, i64>) -> i64 {
    *registers.get(&r).unwrap_or(&0)
}

fn set_register_value(r: RegisterName, v: i64, registers: &mut HashMap<RegisterName, i64>) {
    let entry = registers.entry(r).or_insert(v);
    *entry = v;
}

#[derive(Clone)]
pub struct Program(Vec<Instruction>);

#[derive(Clone, Debug, PartialEq, Copy)]
enum Parameter {
    Register(RegisterName),
    Value(i64),
}

type RegisterName = char;

#[derive(Clone, Debug, PartialEq, Copy)]
enum Instruction {
    Set(RegisterName, Parameter),
    Sound(Parameter),
    Add(RegisterName, Parameter),
    Multiply(RegisterName, Parameter),
    Modulo(RegisterName, Parameter),
    RecoverWhenNotZero(RegisterName),
    JumpGreaterThanZero(Parameter, Parameter),
    JumpNotZero(Parameter, Parameter),
    Sub(RegisterName, Parameter),
}

fn parse_register(s: &str) -> Result<RegisterName, ()> {
    char::from_str(s).map_err(|_| ())
}

fn parse_parameter(s: &str) -> Result<Parameter, ()> {
    if let Ok(value) = i64::from_str(s) {
        Ok(Parameter::Value(value))
    } else if let Ok(register) = parse_register(s) {
        Ok(Parameter::Register(register))
    } else {
        Err(())
    }
}

fn parse_program(code: &str) -> Result<Program, ()> {
    code.lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| Instruction::from_str(line))
        .sum()
}


impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Instruction, ()> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        if words.len() < 2 {
            return Err(());
        }

        match words[0] {
            "snd" => {
                let frequency = parse_parameter(words[1])?;
                Ok(Instruction::Sound(frequency))
            }
            "set" => if words.len() < 3 {
                Err(())
            } else {
                let target = parse_register(words[1])?;
                let value = parse_parameter(words[2])?;
                Ok(Instruction::Set(target, value))
            },
            "add" => if words.len() < 3 {
                Err(())
            } else {
                let target = parse_register(words[1])?;
                let value = parse_parameter(words[2])?;
                Ok(Instruction::Add(target, value))
            },
            "sub" => if words.len() < 3 {
                Err(())
            } else {
                let target = parse_register(words[1])?;
                let value = parse_parameter(words[2])?;
                Ok(Instruction::Sub(target, value))
            },
            "mul" => if words.len() < 3 {
                Err(())
            } else {
                let target = parse_register(words[1])?;
                let value = parse_parameter(words[2])?;
                Ok(Instruction::Multiply(target, value))
            },
            "mod" => if words.len() < 3 {
                Err(())
            } else {
                let target = parse_register(words[1])?;
                let value = parse_parameter(words[2])?;
                Ok(Instruction::Modulo(target, value))
            },
            "rcv" => {
                let target = parse_register(words[1])?;
                Ok(Instruction::RecoverWhenNotZero(target))
            }
            "jgz" => if words.len() < 3 {
                Err(())
            } else {
                let check = parse_parameter(words[1])?;
                let offset = parse_parameter(words[2])?;
                Ok(Instruction::JumpGreaterThanZero(check, offset))
            },
            "jnz" => if words.len() < 3 {
                Err(())
            } else {
                let check = parse_parameter(words[1])?;
                let offset = parse_parameter(words[2])?;
                Ok(Instruction::JumpNotZero(check, offset))
            },
            _ => Err(()),
        }
    }
}

impl Sum<Instruction> for Program {
    fn sum<I>(iter: I) -> Program
    where
        I: Iterator<Item = Instruction>,
    {
        Program(iter.collect())
    }
}


#[cfg(test)]
mod tests {
    mod parse {
        use super::super::*;

        #[test]
        fn test_parse_18_sample_program() {
            let code = r"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";
            let program = parse_program(code);
            assert!(program.is_ok());
            let program = program.unwrap().0;
            assert_eq!(program[0], Instruction::Set('a', Parameter::Value(1)));
            assert_eq!(program[1], Instruction::Add('a', Parameter::Value(2)));
            assert_eq!(
                program[2],
                Instruction::Multiply('a', Parameter::Register('a'))
            );
            assert_eq!(program[3], Instruction::Modulo('a', Parameter::Value(5)));
            assert_eq!(program[4], Instruction::Sound(Parameter::Register('a')));
            assert_eq!(program[5], Instruction::Set('a', Parameter::Value(0)));
            assert_eq!(program[6], Instruction::RecoverWhenNotZero('a'));
            assert_eq!(
                program[7],
                Instruction::JumpGreaterThanZero(Parameter::Register('a'), Parameter::Value(-1))
            );
            assert_eq!(program[8], Instruction::Set('a', Parameter::Value(1)));
            assert_eq!(
                program[9],
                Instruction::JumpGreaterThanZero(Parameter::Register('a'), Parameter::Value(-2))
            );
        }

        #[test]
        fn test_run_18_sample_program() {
            let code = r"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";
            let program = parse_program(code);
            assert!(program.is_ok());
            let (tx, rx) = channel();
            assert_eq!(
                run_program(&program.unwrap(), RunMode::Day18PartOne, rx, tx, 0),
                Ok(4)
            );
        }

        #[test]
        fn test_run_sample_program_for_18_part_two() {
            let code = r"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
";

            let result = run_for_day_18_part2(code);

            assert_eq!(result, Ok(3));
        }
    }
}
