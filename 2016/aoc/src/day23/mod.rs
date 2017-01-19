use super::assembunny::interpreter::Interpreter;
use super::assembunny::ast::Register;
use super::assembunny::parser;

pub fn do_day23(input: &str) {
    let instructions =
        input.lines().filter_map(|line| parser::parse_line(line)).collect::<Vec<_>>();
    {
        let mut interpreter = Interpreter::new(instructions.clone());
        interpreter.set_register(Register::A, 7);
        interpreter.run();
        println!("Register A contains {}", interpreter.value_of(Register::A));
    }
    {
        let mut interpreter = Interpreter::new(instructions);
        interpreter.set_register(Register::A, 12);
        interpreter.run();
        println!("Register A contains {}", interpreter.value_of(Register::A));
    }
}


#[test]
fn test_sample_program() {
    let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
    let instructions =
        input.lines().filter_map(|line| parser::parse_line(line)).collect::<Vec<_>>();
    let mut interpreter = Interpreter::new(instructions);
    interpreter.enable_trace();
    interpreter.run();
    assert_eq!(interpreter.value_of(Register::A), 3);
}
