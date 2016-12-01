extern crate regex;
#[macro_use]
extern crate lazy_static;

mod instructions;
mod parse;
mod taxicabgeometry;

use std::collections::HashMap;
use taxicabgeometry::{Heading, Offset, explode_step};

fn main() {
    let input = "R1, R3, L2, L5, L2, L1, R3, L4, R2, L2, L4, R2, L1, R1, L2, R3, L1, L4, R2, L5, \
                 R3, R4, L1, R2, L1, R3, L4, R5, L4, L5, R5, L3, R2, L3, L3, R1, R3, L4, R2, R5, \
                 L4, R1, L1, L1, R5, L2, R1, L2, R188, L5, L3, R5, R1, L2, L4, R3, R5, L3, R3, \
                 R45, L4, R4, R72, R2, R3, L1, R1, L1, L1, R192, L1, L1, L1, L4, R1, L2, L5, L3, \
                 R5, L3, R3, L4, L3, R1, R4, L2, R2, R3, L5, R3, L1, R1, R4, L2, L3, R1, R3, L4, \
                 L3, L4, L2, L2, R1, R3, L5, L1, R4, R2, L4, L1, R3, R3, R1, L5, L2, R4, R4, R2, \
                 R1, R5, R5, L4, L1, R5, R3, R4, R5, R3, L1, L2, L4, R1, R4, R5, L2, L3, R4, L4, \
                 R2, L2, L4, L2, R5, R1, R4, R3, R5, L4, L4, L5, L5, R3, R4, L1, L3, R2, L2, R1, \
                 L3, L5, R5, R5, R3, L4, L2, R4, R5, R1, R4, L3";
    let steps = parse::parse(input);

    let mut allmoves: Vec<Heading> = vec![];

    for step in steps.iter() {
        let mut def_heading = Heading::North;
        let current_heading = allmoves.last_mut().unwrap_or(&mut def_heading).clone();
        let new_moves = explode_step(&current_heading, step);
        allmoves.extend(new_moves.iter().cloned());
    }

    // now calculate every place visited in every single step

    let mut alloffsets = vec![Offset::new()];
    for m in allmoves.iter() {
        let current_offset = alloffsets.last_mut().unwrap().clone();
        let new_offset = current_offset.move_blocks(&m, &1);
        alloffsets.push(new_offset);
    }

    // now find the first state which occurs twice
    let mut states = HashMap::new();

    for state in alloffsets.iter() {
        let entry = states.entry(state).or_insert(0);
        *entry += 1;

        if *entry == 2 {
            println!("First state with two visits is {:?}", state.distance());
            break;
        }
    }
}
