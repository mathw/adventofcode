use super::ast::*;
use std::collections::HashMap;

pub fn run<'a>(program: &'a Program<'a>) -> HashMap<&'a str, i32> {
    let mut registers = HashMap::new();

    for instruction in program.instructions.iter() {
        if run_conditional(&instruction.condition, &registers) {
            run_action(&instruction.action, &mut registers);
        }
    }

    registers
}

fn run_conditional<'a>(condition: &'a Condition<'a>, registers: &HashMap<&'a str, i32>) -> bool {
    let reg_value = registers.get(condition.register).unwrap_or(&0);

    match condition.op {
        Operator::EqualTo => *reg_value == condition.value,
        Operator::NotEqualTo => *reg_value != condition.value,
        Operator::GreaterThan => *reg_value > condition.value,
        Operator::GreaterThanOrEqualTo => *reg_value >= condition.value,
        Operator::LessThan => *reg_value < condition.value,
        Operator::LessThanOrEqualTo => *reg_value <= condition.value,
    }
}

fn run_action<'a>(action: &'a Action<'a>, registers: &mut HashMap<&'a str, i32>) {
    let target = registers.entry(action.target).or_insert(0);

    match action.op {
        ActionOp::Inc => *target += action.value,
        ActionOp::Dec => *target -= action.value,
    }
}