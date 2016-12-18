mod ast;
mod parser;
mod interpreter;

use self::interpreter::Interpreter;
use self::ast::Register;

pub fn do_day12(input: &str) {
    let instructions =
        input.lines().filter_map(|line| parser::parse_line(line)).collect::<Vec<_>>();

    {
        let mut interpreter = Interpreter::new(instructions.clone());
        interpreter.run();

        println!("The value left in register A is {}",
                 interpreter.value_of(Register::A));
    }

    {
        let mut interpreter = Interpreter::new(instructions);
        interpreter.set_register(Register::C, 1);
        println!("Initialised C to 1.");
        interpreter.run();
        println!("The value left in register A is {}",
                 interpreter.value_of(Register::A));
    }

}
