use nom::IResult;
use std::str;
use super::instructions::{Instruction, Target};
use nom_helpers::as_u32;

named!(bot<Target>,
       ws!(do_parse!(tag!("bot") >> num: as_u32 >> (Target::Bot(num)))));
named!(output<Target>,
       ws!(do_parse!(tag!("output") >> num: as_u32 >> (Target::Output(num)))));
named!(target<Target>, alt!(bot | output));
named!(give_value<Instruction>,
       ws!(do_parse!(tag!("value") >> chip_value: as_u32 >> tag!("goes") >> tag!("to") >>
                     tag!("bot") >> bot_num: as_u32 >>
                     (Instruction::give(chip_value, bot_num)))));
named!(decide<Instruction>,
       ws!(do_parse!(tag!("bot") >> source_bot: as_u32 >> tag!("gives") >> tag!("low") >>
                     tag!("to") >> low_dest: target >>
                     tag!("and") >> tag!("high") >>
                     tag!("to") >> high_dest: target >>
                     (Instruction::decide(source_bot, low_dest, high_dest)))));

named!(instruction<Instruction>, alt!(decide | give_value));

pub fn parse_instruction(line: &str) -> Option<Instruction> {
    match instruction(line.as_bytes()) {
        IResult::Done(_, instr) => Some(instr),
        _ => None,
    }
}

#[test]
fn test_u32() {
    let src = b"678";
    let badsrc = b"ajd";

    assert_done_and_eq!(as_u32(src), 678);
    assert_error!(as_u32(badsrc));
}

#[test]
fn test_bot() {
    let src = b"bot 689";
    let badsrc = b"bot alex";
    let badsrc2 = b"output 4";

    assert_done_and_eq!(bot(src), Target::Bot(689));
    assert_error!(bot(badsrc));
    assert_error!(bot(badsrc2));
}

#[test]
fn test_output() {
    let src = b"output 689";
    let badsrc = b"output alex";
    let badsrc2 = b"bot 4";

    assert_done_and_eq!(output(src), Target::Output(689));
    assert_error!(output(badsrc));
    assert_error!(output(badsrc2));
}

#[test]
fn test_target() {
    let botsrc = b"bot 56";
    let outputsrc = b"output 32";
    let nonsensesrc = b"jasdiiwoo";

    assert_done_and_eq!(target(botsrc), Target::Bot(56));
    assert_done_and_eq!(target(outputsrc), Target::Output(32));
    assert_error!(target(nonsensesrc));
}

#[test]
fn test_give() {
    let src = b"value 32 goes to bot 56";
    let badsrc = b"value 32 goes to output 3";

    assert_done_and_eq!(give_value(src), Instruction::Give { value: 32, to: 56 });
    assert_error!(give_value(badsrc));
}

#[test]
fn test_decide() {
    let src = b"bot 3 gives low to output 3 and high to bot 2";

    assert_done_and_eq!(decide(src), Instruction::Decide { bot: 3, low: Target::Output(3), high: Target::Bot(2) });
}
