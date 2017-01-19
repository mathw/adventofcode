use super::ast::{Instruction, Register, RegOrInt};
use nom::IResult;
use nom_helpers::as_i32;

named!(register_a<Register>, do_parse!(
    tag!("a") >>
    (Register::A)
));

named!(register_b<Register>, do_parse!(
    tag!("b") >>
    (Register::B)
));

named!(register_c<Register>, do_parse!(
    tag!("c") >>
    (Register::C)
));

named!(register_d<Register>, do_parse!(
    tag!("d") >>
    (Register::D)
));

named!(register<Register>, alt!(
    register_a | register_b | register_c | register_d
));

named!(inc<Instruction>, ws!(do_parse!(
    tag!("inc") >>
    reg: register >>
    (Instruction::Inc(reg))
)));

named!(dec<Instruction>, ws!(do_parse!(
    tag!("dec") >>
    reg: register >>
    (Instruction::Dec(reg))
)));

named!(intsrc<RegOrInt>, do_parse!(
    i: as_i32 >>
    (RegOrInt::Int(i))
));

named!(registersrc<RegOrInt>, do_parse!(
    r: register >>
    (RegOrInt::Reg(r))
));

named!(regorint<RegOrInt>, alt!(
    intsrc | registersrc
));

named!(copy<Instruction>, ws!(do_parse!(
    tag!("cpy") >>
    src: regorint >>
    target: register >>
    (Instruction::Copy { from: src, to: target })
)));

named!(jnz<Instruction>, ws!(do_parse!(
    tag!("jnz") >>
    r: regorint >>
    o: regorint >>
    (Instruction::Jump { test: r, offset: o })
)));

named!(tgl<Instruction>, ws!(do_parse!(
    tag!("tgl") >>
    o: regorint >>
    (Instruction::Toggle(o))
)));

named!(instruction<Instruction>, alt!(
    inc | dec | copy | jnz | tgl
));

pub fn parse_line(line: &str) -> Option<Instruction> {
    match instruction(line.as_bytes()) {
        IResult::Done(_, i) => Some(i),
        _ => None,
    }
}

#[test]
fn test_register_parsers() {
    assert_done_and_eq!(register(b"a"), Register::A);
    assert_done_and_eq!(register(b"b"), Register::B);
    assert_done_and_eq!(register(b"c"), Register::C);
    assert_done_and_eq!(register(b"d"), Register::D);
    assert_error!(register(b"e"));
}

#[test]
fn test_instruction_parsers() {
    assert_done_and_eq!(instruction(b"inc d"), Instruction::Inc(Register::D));
    assert_done_and_eq!(instruction(b"dec a"), Instruction::Dec(Register::A));
    assert_done_and_eq!(instruction(b"cpy 3 d"),
     Instruction::Copy { from: RegOrInt::Int(3), to: Register::D});
    assert_done_and_eq!(instruction(b"cpy a d"),
     Instruction::Copy { from: RegOrInt::Reg(Register::A), to: Register::D});
    assert_done_and_eq!(instruction(b"jnz a 7"),
     Instruction::Jump { test: RegOrInt::Reg(Register::A), offset: RegOrInt::Int(7)});
    assert_done_and_eq!(instruction(b"jnz 6 6"),
     Instruction::Jump { test: RegOrInt::Int(6), offset: RegOrInt::Int(6)});
    assert_done_and_eq!(instruction(b"tgl 5"), Instruction::Toggle(RegOrInt::Int(5)));
}
