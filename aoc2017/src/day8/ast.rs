#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub instructions: Vec<Instruction<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>) -> Program<'a> {
        Program { instructions: instructions }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    pub condition: Condition<'a>,
    pub action: Action<'a>,
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
    pub register: &'a str,
    pub op: Operator,
    pub value: i32,
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
    pub target: &'a str,
    pub op: ActionOp,
    pub value: i32,
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
