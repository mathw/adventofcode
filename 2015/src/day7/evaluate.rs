use super::instruction::Instruction;
use std::collections::HashMap;

pub fn evaluate(instructions: &Vec<Instruction>) -> HashMap<String, u16> {
    let mut wirestates = HashMap::new();

    for instruction in instructions {
        run_instruction(&mut wirestates, &instruction);
    }

    wirestates
}

fn set_wire_value(states: &mut HashMap<String, u16>, wire: &String, value: u16) {
    let entry = states.entry(wire.clone()).or_insert(0);
    *entry = value;
}

fn run_instruction(states: &mut HashMap<String, u16>, instruction: &Instruction) {
    match instruction {
        &Instruction::Signal { value, ref target } => set_wire_value(states, &target, value),
        &Instruction::And { ref a, ref b, ref target } => run_and(states, &a, &b, &target),
        &Instruction::Or { ref a, ref b, ref target } => run_or(states, &a, &b, &target),
        &Instruction::Shift { ref input, distance, ref target } => {
            run_shift(states, &input, distance, &target)
        }
        &Instruction::Not { ref input, ref target } => run_not(states, &input, &target),
    }
}

fn run_and(states: &mut HashMap<String, u16>, a: &String, b: &String, target: &String) {
    let a = get_value(states, a);
    let b = get_value(states, b);

    set_wire_value(states, target, a & b);
}

fn run_or(states: &mut HashMap<String, u16>, a: &String, b: &String, target: &String) {
    let a = get_value(states, a);
    let b = get_value(states, b);

    set_wire_value(states, target, a | b);
}

fn run_shift(states: &mut HashMap<String, u16>, input: &String, distance: i32, target: &String) {
    let input = get_value(states, input);

    let value = if distance < 0 {
        input << distance.abs()
    } else {
        input >> distance
    };

    set_wire_value(states, target, value);
}

fn run_not(states: &mut HashMap<String, u16>, input: &String, target: &String) {
    let input = get_value(states, input);

    set_wire_value(states, target, !input);
}

fn get_value(states: &HashMap<String, u16>, wire: &String) -> u16 {
    states.get(wire).unwrap().clone()
}