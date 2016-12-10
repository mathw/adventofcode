mod instructions;
mod parser;
mod bot;
mod interpreter;
mod state;

use self::state::State;

fn make_state(input: &str) -> State {
    let instructions = input.lines().filter_map(|line| parser::parse_instruction(line));
    let mut state = State::new();
    for instruction in instructions {
        state.act_on(instruction);
    }

    state
}

pub fn run_until_bot_holds(input: &str, values: (u32, u32)) -> Option<u32> {
    let mut state = make_state(input);
    state.run_until_bot_holds(values)
}

fn run_all(input: &str) -> State {
    let mut state = make_state(input);
    state.run_all_decisions();
    state
}

pub fn do_day10(input: &str) {
    let botnum = run_until_bot_holds(input, (61, 17));

    println!("\n\n\n==============");
    let p2state = run_all(input);
    let bin0 = p2state.get_output_bin(0)[0];
    let bin1 = p2state.get_output_bin(1)[0];
    let bin2 = p2state.get_output_bin(2)[0];

    if botnum.is_some() {
        println!("Bot {} compares 61 to 17", botnum.unwrap());
    } else {
        println!("No bot compares 61 to 17");
    }

    println!("0 x 1 x 2 = {}", bin0 * bin1 * bin2);
}

#[test]
fn test_rickety_machine() {
    let instructions = r"
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
    ";

    let botnum = run_until_bot_holds(instructions, (5, 2));

    assert_eq!(botnum, Some(2));
}
