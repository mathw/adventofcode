use crate::intcode::memory::Memory;
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::Rem;
use std::str::FromStr;

mod memory;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program<N> {
    locations: Vec<N>,
}

impl<N> FromStr for Program<N>
where
    N: FromStr,
{
    type Err = <N as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|n| N::from_str(n.trim()))
            .collect::<Result<Vec<N>, Self::Err>>()
            .map(|v| Program { locations: v })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State<N> {
    NeedsInput,
    ProvidedOutput(N),
    Completed,
}

pub struct RunState<N> {
    pub state: State<N>,
    runner: ProgramRunner<N>,
}

impl<N> RunState<N>
where
    N: Copy
        + PartialOrd
        + From<i32>
        + Display
        + Rem<Output = N>
        + Add<Output = N>
        + Mul<Output = N>,
    usize: TryFrom<N>,
{
    pub fn resume(self) -> RunState<N> {
        RunState::next(self.runner)
    }

    pub fn resume_with_input(self, input: N) -> RunState<N> {
        let mut runner = self.runner;
        runner.provide_input(input);
        RunState::next(runner)
    }

    fn next(mut runner: ProgramRunner<N>) -> RunState<N> {
        loop {
            match runner.run_until_state_change() {
                ProgramState::Completed => {
                    return RunState {
                        state: State::Completed,
                        runner,
                    }
                }
                ProgramState::NeedsInput => {
                    return RunState {
                        state: State::NeedsInput,
                        runner,
                    }
                }
                ProgramState::ProvidedOutput(o) => {
                    return RunState {
                        state: State::ProvidedOutput(o),
                        runner,
                    }
                }
                ProgramState::Running => continue,
                ProgramState::NotStarted => panic!("Cannot transition into NotStarted"),
            }
        }
    }
}

impl<N> Program<N>
where
    N: Copy
        + PartialOrd
        + From<i32>
        + Display
        + Rem<Output = N>
        + Add<Output = N>
        + Mul<Output = N>,
    usize: TryFrom<N>,
{
    pub fn run(&mut self, inputs: &Vec<N>) -> Vec<N> {
        let (locations, outputs) = self.run_core(inputs);
        self.locations = locations;

        outputs
    }

    pub fn run_pure(&self, inputs: &Vec<N>) -> Vec<N> {
        self.run_core(inputs).1
    }

    pub fn run_until_needs_interaction(&self) -> RunState<N>
    where
        N: Copy,
    {
        let runner = ProgramRunner::new(self.locations.clone());
        let runstate = RunState {
            state: State::Completed,
            runner: runner,
        };
        runstate.resume()
    }

    fn run_core(&self, inputs: &Vec<N>) -> (Vec<N>, Vec<N>)
    where
        N: Copy,
    {
        let mut inputs = inputs.iter().rev().cloned().collect::<Vec<N>>();
        let mut outputs = Vec::new();

        let mut runstate = self.run_until_needs_interaction();

        loop {
            match runstate.state {
                State::Completed => break,
                State::NeedsInput => {
                    runstate = runstate
                        .resume_with_input(inputs.pop().expect("Not enough inputs provideed"))
                }
                State::ProvidedOutput(o) => {
                    outputs.push(o);
                    runstate = runstate.resume()
                }
            }
        }

        (runstate.runner.memory.as_vector(), outputs)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ProgramState<N> {
    NotStarted,
    Running,
    NeedsInput,
    ProvidedOutput(N),
    Completed,
}

struct ProgramRunner<N> {
    memory: Memory<N>,
    program_counter: usize,
    relative_offset: usize,
    inputs: Vec<N>,
    state: ProgramState<N>,
}

impl<N> ProgramRunner<N>
where
    N: From<i32>
        + Rem<Output = N>
        + Add<Output = N>
        + Mul<Output = N>
        + PartialOrd
        + Display
        + Copy,
    usize: TryFrom<N>,
{
    fn new(locations: Vec<N>) -> ProgramRunner<N> {
        ProgramRunner {
            memory: Memory::from(locations),
            program_counter: 0,
            relative_offset: 0,
            inputs: Vec::new(),
            state: ProgramState::NotStarted,
        }
    }

    fn run_until_state_change(&mut self) -> ProgramState<N> {
        while self.opcode() == ProgramState::Running {}
        self.state
    }

    fn current(&self) -> N {
        self.memory.get(self.program_counter)
    }

    fn to_usize_or_panic(n: N) -> usize {
        match usize::try_from(n) {
            Ok(u) => u,
            Err(_) => panic!("This N must be convertable to usize"),
        }
    }

    fn current_opcode(&self) -> usize {
        Self::to_usize_or_panic(self.current() % 100i32.into())
    }

    fn trinary_parameter_modes(&self) -> (Mode, Mode, Mode) {
        if self.current() > 100i32.into() {
            str_to_trinary_modes(&self.current().to_string())
        } else {
            (Mode::Position, Mode::Position, Mode::Position)
        }
    }

    fn binary_parameter_modes(&self) -> (Mode, Mode) {
        if self.current() > 100i32.into() {
            str_to_binary_modes(&self.current().to_string())
        } else {
            (Mode::Position, Mode::Position)
        }
    }

    fn unary_parameter_mode(&self) -> Mode {
        if self.current() > 100i32.into() {
            str_to_unary_mode(&self.current().to_string())
        } else {
            Mode::Position
        }
    }

    fn opcode(&mut self) -> ProgramState<N> {
        self.state = match self.current_opcode() {
            99 => ProgramState::Completed,
            1 => self.binary_operation(|a, b| a + b),
            2 => self.binary_operation(|a, b| a * b),
            3 => self.input(),
            4 => self.output(),
            5 => self.jump_if(true),
            6 => self.jump_if(false),
            7 => self.comparative(|a, b| a < b),
            8 => self.comparative(|a, b| a == b),
            9 => self.set_relative_offset(),
            op => panic!("Unknown opcode {}", op),
        };

        self.state
    }

    fn parameter_value(&self, offset: usize, mode: Mode) -> N {
        let parameter_index = self.program_counter + offset;
        let value_index = match mode {
            Mode::Immediate => self.program_counter + offset,
            Mode::Position => Self::to_usize_or_panic(self.memory.get(parameter_index)),
            Mode::Relative => {
                Self::to_usize_or_panic(self.memory.get(parameter_index)) + self.relative_offset
            }
        };
        self.memory.get(value_index)
    }

    fn output_parameter_write_location(&self, offset: usize, mode: Mode) -> usize {
        let parameter_index = self.program_counter + offset;
        Self::to_usize_or_panic(match mode {
            Mode::Immediate | Mode::Position => self.memory.get(parameter_index),
            Mode::Relative => self
                .memory
                .get(Self::to_usize_or_panic(self.memory.get(parameter_index))),
        })
    }

    fn binary_operation<O>(&mut self, operation: O) -> ProgramState<N>
    where
        O: Fn(N, N) -> N,
    {
        let (mode1, mode2, mode3) = self.trinary_parameter_modes();
        let result_position = self.output_parameter_write_location(3, mode3);
        let first_argument = self.parameter_value(1, mode1);
        let second_argument = self.parameter_value(2, mode2);

        self.memory
            .set(result_position, operation(first_argument, second_argument));

        self.advance(4);

        ProgramState::Running
    }

    fn provide_input(&mut self, input: N) {
        self.inputs.push(input)
    }

    fn input(&mut self) -> ProgramState<N> {
        if self.inputs.is_empty() {
            ProgramState::NeedsInput
        } else {
            let mode = self.unary_parameter_mode();
            let write_location = self.output_parameter_write_location(1, mode);
            let input = self.inputs.pop().expect("Cannot run input: no more inputs");
            self.memory.set(write_location, input);
            self.advance(2);
            ProgramState::Running
        }
    }

    fn output(&mut self) -> ProgramState<N> {
        let first_argument = self.parameter_value(1, self.unary_parameter_mode());
        self.advance(2);
        ProgramState::ProvidedOutput(first_argument)
    }

    fn jump_if(&mut self, want_true: bool) -> ProgramState<N> {
        let (mode1, mode2) = self.binary_parameter_modes();
        let first = self.parameter_value(1, mode1);
        let second = Self::to_usize_or_panic(self.parameter_value(2, mode2));

        if want_true {
            if first != 0i32.into() {
                self.jump(second);
                return ProgramState::Running;
            }
        } else if first == 0i32.into() {
            self.jump(second);
            return ProgramState::Running;
        }

        self.advance(3);
        ProgramState::Running
    }

    fn advance(&mut self, offset: usize) {
        self.program_counter += offset
    }

    fn jump(&mut self, target: usize) {
        self.program_counter = target
    }

    fn comparative<F>(&mut self, compare: F) -> ProgramState<N>
    where
        F: Fn(N, N) -> bool,
    {
        self.binary_operation(|a, b| {
            if compare(a, b) {
                1i32.into()
            } else {
                0i32.into()
            }
        })
    }

    fn set_relative_offset(&mut self) -> ProgramState<N> {
        let mode = self.unary_parameter_mode();
        let value = self.parameter_value(1, mode);

        self.relative_offset = Self::to_usize_or_panic(value);

        ProgramState::Running
    }
}

impl<N> Index<usize> for Program<N> {
    type Output = N;

    fn index(&self, i: usize) -> &Self::Output {
        self.locations.index(i)
    }
}

impl<N> IndexMut<usize> for Program<N> {
    fn index_mut(&mut self, i: usize) -> &mut <Self as Index<usize>>::Output {
        self.locations.index_mut(i)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

fn char_to_mode(c: char) -> Mode {
    match c {
        '0' => Mode::Position,
        '1' => Mode::Immediate,
        '2' => Mode::Relative,
        _ => panic!("unknown mode character {}", c),
    }
}

fn str_to_trinary_modes(s: &str) -> (Mode, Mode, Mode) {
    let mut nums = s.chars().rev().skip(2).chain(vec!['0', '0', '0']);
    (
        char_to_mode(nums.next().unwrap()),
        char_to_mode(nums.next().unwrap()),
        char_to_mode(nums.next().unwrap()),
    )
}

fn str_to_binary_modes(s: &str) -> (Mode, Mode) {
    let mut nums = s.chars().rev().skip(2).chain(vec!['0', '0']);
    (
        char_to_mode(nums.next().unwrap()),
        char_to_mode(nums.next().unwrap()),
    )
}

fn str_to_unary_mode(s: &str) -> Mode {
    let mut nums = s.chars().rev().skip(2).chain(vec!['0', '0']);
    char_to_mode(nums.next().unwrap())
}

#[test]
fn test_str_to_binary_modes() {
    assert_eq!(
        str_to_binary_modes("1101"),
        (Mode::Immediate, Mode::Immediate)
    );
    assert_eq!(
        str_to_binary_modes("1001"),
        (Mode::Position, Mode::Immediate)
    );
    assert_eq!(
        str_to_binary_modes("0001"),
        (Mode::Position, Mode::Position)
    );
    assert_eq!(str_to_binary_modes("1"), (Mode::Position, Mode::Position));
}

#[test]
fn test_add_immediate() {
    // add 2 + 3 and store in 0
    let mut program = Program::<i32>::from_str("1101,2,3,0,99").unwrap();
    program.run(&vec![]);
    assert_eq!(program[0], 5);
}

#[test]
fn test_add_position() {
    // add positions 5 and 6 and store in 0
    let mut program = Program::<i32>::from_str("1,5,6,0,99,1,3").unwrap();
    program.run(&vec![]);
    assert_eq!(program[0], 4);
}

#[test]
fn test_from_str_happy() {
    let input = "2,4,5,219 ,00,2920";
    let program = Program::<i32>::from_str(input).expect("This should not fail");
    assert_eq!(program.locations, vec![2, 4, 5, 219, 0, 2920]);
}

#[test]
fn test_from_str_unhappy() {
    let input = "2,4,potato,219,00,2920";
    let program = Program::<i32>::from_str(input);
    assert!(program.is_err());
    assert_eq!(program.err(), usize::from_str("potato").err());
}

#[test]
fn test_input() {
    let mut program = Program::<i32>::from_str("3,3,99,5,22").unwrap();
    program.run(&vec![9, 8]);
    assert_eq!(program[3], 9);
}

#[test]
fn test_output() {
    let program = Program::<i32>::from_str("4,3,99,5").unwrap();
    let outputs = program.run_pure(&vec![]);
    assert_eq!(outputs, vec![5]);
}

#[test]
fn test_run_sample() {
    let mut program = Program::<i64>::from_str("1,9,10,3,2,3,11,0,99,30,40,50").unwrap();
    program.run(&mut vec![]);
    assert_eq!(program[0], 3500);
}

#[test]
fn test_output_immediate() {
    let program = Program::<i128>::from_str("104, 2, 99, 5, 22").unwrap();
    let outputs = program.run_pure(&vec![]);
    assert_eq!(outputs, vec![2]);
}

#[test]
fn test_day5_part2_sample1() {
    // checks if input == 8, outputs 1 if so, 0 if not
    let program =
        Program::<i32>::from_str("3,9,8,9,10,9,4,9,99,-1,8").expect("Program should parse");

    // check == 8
    let outputs = program.run_pure(&mut vec![8]);
    assert_eq!(outputs, vec![1]);

    // check != 8
    let outputs = program.run_pure(&mut vec![6]);
    assert_eq!(outputs, vec![0]);
}

#[test]
fn test_day5_part2_sample3() {
    // checks if input == 8, outputs 1 if so, 0 if not
    let program = Program::<i32>::from_str("3,3,1108,-1,8,3,4,3,99").expect("Program should parse");

    // check == 8
    let outputs = program.run_pure(&mut vec![8]);
    assert_eq!(outputs, vec![1]);

    // check != 8
    let outputs = program.run_pure(&mut vec![6]);
    assert_eq!(outputs, vec![0]);
}

#[test]
fn test_day5_part2_big_sample() {
    // takes one input
    // outputs 999 if input < 8, 1000 if input == 8, 1001 if input > 8
    // uses jump instructions to do it
    let program = Program::<i32>::from_str(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
    )
    .expect("Program should parse");

    // check == 8
    let outputs = program.run_pure(&mut vec![8]);
    assert_eq!(outputs, vec![1000]);

    // check < 8
    let outputs = program.run_pure(&mut vec![7]);
    assert_eq!(outputs, vec![999]);

    // check > 8
    let outputs = program.run_pure(&mut vec![9]);
    assert_eq!(outputs, vec![1001]);
}

#[test]
fn test_out_of_bounds_output() {
    let program = Program::<i32>::from_str("4, 0, 4, 67, 99").expect("Program should parse");

    // should output location 0 (4), then location 67 (0)
    let outputs = program.run_pure(&Vec::new());

    assert_eq!(outputs, vec![4, 0]);
}

#[test]
fn test_out_of_bounds_write() {
    let program = Program::<i32>::from_str("01101,2,3,7,4,7,99").expect("Program should parse");

    // calculate 2 + 3 and store in #7, then output #7
    let outputs = program.run_pure(&Vec::new());

    assert_eq!(outputs, vec![5]);
}
