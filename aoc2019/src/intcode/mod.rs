use std::num::ParseIntError;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    locations: Vec<i32>,
}

impl FromStr for Program {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|n| i32::from_str(n.trim()))
            .collect::<Result<Vec<i32>, Self::Err>>()
            .map(|v| Program { locations: v })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    NeedsInput,
    ProvidedOutput(i32),
    Completed,
}

pub struct RunState {
    pub state: State,
    runner: ProgramRunner,
}

impl RunState {
    pub fn resume(self) -> RunState {
        RunState::next(self.runner)
    }

    pub fn resume_with_input(self, input: i32) -> RunState {
        let mut runner = self.runner;
        runner.provide_input(input);
        RunState::next(runner)
    }

    fn next(mut runner: ProgramRunner) -> RunState {
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

impl Program {
    pub fn run(&mut self, inputs: &Vec<i32>) -> Vec<i32> {
        let (locations, outputs) = self.run_core(inputs);
        self.locations = locations;

        outputs
    }

    pub fn run_pure(&self, inputs: &Vec<i32>) -> Vec<i32> {
        self.run_core(inputs).1
    }

    pub fn run_until_needs_interaction(&self) -> RunState {
        let runner = ProgramRunner::new(self.locations.clone());
        let runstate = RunState {
            state: State::Completed,
            runner: runner,
        };
        runstate.resume()
    }

    fn run_core(&self, inputs: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut inputs = inputs.iter().rev().cloned().collect::<Vec<i32>>();
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

        (runstate.runner.locations, outputs)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ProgramState {
    NotStarted,
    Running,
    NeedsInput,
    ProvidedOutput(i32),
    Completed,
}

struct ProgramRunner {
    locations: Vec<i32>,
    program_counter: usize,
    inputs: Vec<i32>,
    state: ProgramState,
}

impl ProgramRunner {
    fn new(locations: Vec<i32>) -> ProgramRunner {
        ProgramRunner {
            locations,
            program_counter: 0,
            inputs: Vec::new(),
            state: ProgramState::NotStarted,
        }
    }

    fn run_until_state_change(&mut self) -> ProgramState {
        while self.opcode() == ProgramState::Running {}
        self.state
    }

    fn current(&self) -> i32 {
        self.locations[self.program_counter]
    }

    fn current_opcode(&self) -> usize {
        (self.current() % 100) as usize
    }

    fn binary_parameter_modes(&self) -> (Mode, Mode) {
        if self.current() > 100 {
            str_to_binary_modes(&self.current().to_string())
        } else {
            (Mode::Position, Mode::Position)
        }
    }

    fn unary_parameter_mode(&self) -> Mode {
        if self.current() > 100 {
            str_to_unary_mode(&self.current().to_string())
        } else {
            Mode::Position
        }
    }

    fn opcode(&mut self) -> ProgramState {
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
            op => panic!("Unknown opcode {}", op),
        };

        self.state
    }

    fn argument_value(&self, offset: usize, mode: Mode) -> i32 {
        let location_value = self.at_offset(offset);
        match mode {
            Mode::Immediate => location_value,
            Mode::Position => self.locations[location_value as usize],
        }
    }

    fn at_offset(&self, offset: usize) -> i32 {
        self.locations[self.program_counter + offset]
    }

    fn binary_operation<O>(&mut self, operation: O) -> ProgramState
    where
        O: Fn(i32, i32) -> i32,
    {
        let (mode1, mode2) = self.binary_parameter_modes();
        let result_position = self.at_offset(3) as usize;
        let first_argument = self.argument_value(1, mode1);
        let second_argument = self.argument_value(2, mode2);

        self.locations[result_position] = operation(first_argument, second_argument);

        self.advance(4);

        ProgramState::Running
    }

    fn provide_input(&mut self, input: i32) {
        self.inputs.push(input)
    }

    fn input(&mut self) -> ProgramState {
        if self.inputs.is_empty() {
            ProgramState::NeedsInput
        } else {
            let first_argument_position = self.at_offset(1) as usize;
            let input = self.inputs.pop().expect("Cannot run input: no more inputs");
            self.locations[first_argument_position] = input;
            self.advance(2);
            ProgramState::Running
        }
    }

    fn output(&mut self) -> ProgramState {
        let first_argument = self.argument_value(1, self.unary_parameter_mode());
        self.advance(2);
        ProgramState::ProvidedOutput(first_argument)
    }

    fn jump_if(&mut self, want_true: bool) -> ProgramState {
        let (mode1, mode2) = self.binary_parameter_modes();
        let first = self.argument_value(1, mode1);
        let second = self.argument_value(2, mode2) as usize;

        if want_true {
            if first != 0 {
                self.jump(second);
                return ProgramState::Running;
            }
        } else if first == 0 {
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

    fn comparative<F>(&mut self, compare: F) -> ProgramState
    where
        F: Fn(i32, i32) -> bool,
    {
        self.binary_operation(|a, b| if compare(a, b) { 1 } else { 0 })
    }
}

impl Index<usize> for Program {
    type Output = i32;

    fn index(&self, i: usize) -> &Self::Output {
        self.locations.index(i)
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, i: usize) -> &mut <Self as Index<usize>>::Output {
        self.locations.index_mut(i)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

fn char_to_mode(c: char) -> Mode {
    match c {
        '0' => Mode::Position,
        '1' => Mode::Immediate,
        _ => panic!("unknown mode character {}", c),
    }
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
    let mut program = Program::from_str("1101,2,3,0,99").unwrap();
    program.run(&vec![]);
    assert_eq!(program[0], 5);
}

#[test]
fn test_add_position() {
    // add positions 5 and 6 and store in 0
    let mut program = Program::from_str("1,5,6,0,99,1,3").unwrap();
    program.run(&vec![]);
    assert_eq!(program[0], 4);
}

#[test]
fn test_from_str_happy() {
    let input = "2,4,5,219 ,00,2920";
    let program = Program::from_str(input).expect("This should not fail");
    assert_eq!(program.locations, vec![2, 4, 5, 219, 0, 2920]);
}

#[test]
fn test_from_str_unhappy() {
    let input = "2,4,potato,219,00,2920";
    let program = Program::from_str(input);
    assert!(program.is_err());
    assert_eq!(program.err(), usize::from_str("potato").err());
}

#[test]
fn test_input() {
    let mut program = Program::from_str("3,3,99,5,22").unwrap();
    program.run(&vec![9, 8]);
    assert_eq!(program[3], 9);
}

#[test]
fn test_output() {
    let program = Program::from_str("4,3,99,5").unwrap();
    let outputs = program.run_pure(&vec![]);
    assert_eq!(outputs, vec![5]);
}

#[test]
fn test_run_sample() {
    let mut program = Program::from_str("1,9,10,3,2,3,11,0,99,30,40,50").unwrap();
    program.run(&mut vec![]);
    assert_eq!(program[0], 3500);
}

#[test]
fn test_output_immediate() {
    let program = Program::from_str("104, 2, 99, 5, 22").unwrap();
    let outputs = program.run_pure(&vec![]);
    assert_eq!(outputs, vec![2]);
}

#[test]
fn test_day5_part2_sample1() {
    // checks if input == 8, outputs 1 if so, 0 if not
    let program = Program::from_str("3,9,8,9,10,9,4,9,99,-1,8").expect("Program should parse");

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
    let program = Program::from_str("3,3,1108,-1,8,3,4,3,99").expect("Program should parse");

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
    let program = Program::from_str(
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
