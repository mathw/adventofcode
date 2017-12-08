use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    registers: HashSet<&'a str>,
    instructions: Vec<Instruction<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>, registers: HashSet<&'a str>) -> Program<'a> {
        Program {
            registers: registers,
            instructions: instructions,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    condition: Condition<'a>,
    action: Action<'a>,
}

impl<'a> Instruction<'a> {
    pub fn new(condition: Condition<'a>, action: Action<'a>) -> Instruction<'a> {
        Instruction {
            condition: condition,
            action: action,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Condition<'a> {
    register: &'a str,
    op: Operator,
    value: i32,
}

impl<'a> Condition<'a> {
    pub fn new(register: &'a str, op: Operator, value: i32) -> Condition<'a> {
        Condition {
            register: register,
            op: op,
            value: value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Action<'a> {
    target: &'a str,
    op: ActionOp,
    value: i32,
}

impl<'a> Action<'a> {
    pub fn new(target: &'a str, op: ActionOp, value: i32) -> Action<'a> {
        Action {
            target: target,
            op: op,
            value: value,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operator {
    LessThan,
    GreaterThan,
    EqualTo,
    NotEqualTo,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionOp {
    Inc,
    Dec,
}
