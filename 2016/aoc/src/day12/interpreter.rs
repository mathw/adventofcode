use super::ast::{Instruction, Register, RegOrInt};

pub struct Interpreter {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: u32,
    program: Vec<Instruction>,
    trace: bool,
}

impl Interpreter {
    pub fn new(program: Vec<Instruction>) -> Interpreter {
        Interpreter {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            program: program,
            trace: false,
        }
    }

    // pub fn enable_trace(&mut self) {
    //     self.trace = true;
    // }

    pub fn value_of(&self, register: Register) -> i32 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    pub fn set_register(&mut self, register: Register, value: i32) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
        }
    }

    /// Run the next instruction.
    /// Returns false if there was no next instruction
    fn run_next(&mut self) -> bool {
        if self.program.len() <= self.pc as usize {
            // no more instructions
            if self.trace {
                println!("No further instructions.");
            }
            return false;
        }

        if self.trace {
            println!("Retrieving instruction {}", self.pc);
        }
        let next_instruction = self.program[self.pc as usize];

        if self.trace {
            println!("Running {:?}", next_instruction);
        }

        match next_instruction {
            Instruction::Inc(r) => {
                match r {
                    Register::A => self.a += 1,
                    Register::B => self.b += 1,
                    Register::C => self.c += 1,
                    Register::D => self.d += 1,
                };
                self.pc += 1;
                if self.trace {
                    println!("Incremented {:?} which is now {}", r, self.value_of(r));
                }
            }
            Instruction::Dec(r) => {
                match r {
                    Register::A => self.a -= 1,
                    Register::B => self.b -= 1,
                    Register::C => self.c -= 1,
                    Register::D => self.d -= 1,
                };
                self.pc += 1;
                if self.trace {
                    println!("Decremented {:?} which is now {}", r, self.value_of(r));
                }
            }
            Instruction::Copy { from, to } => {
                match from {
                    RegOrInt::Int(i) => {
                        match to {
                            Register::A => self.a = i,
                            Register::B => self.b = i,
                            Register::C => self.c = i,
                            Register::D => self.d = i,
                        }
                        if self.trace {
                            println!("Copied {:?} to {:?} which is now {}",
                                     i,
                                     to,
                                     self.value_of(to));
                        }
                    }
                    RegOrInt::Reg(r) => {
                        match to {
                            Register::A => self.a = self.value_of(r),
                            Register::B => self.b = self.value_of(r),
                            Register::C => self.c = self.value_of(r),
                            Register::D => self.d = self.value_of(r),
                        }
                        if self.trace {
                            println!("Copied {:?} to {:?} which is now {}",
                                     r,
                                     to,
                                     self.value_of(to));
                        }
                    }
                };
                self.pc += 1;
            }
            Instruction::Jump { test, offset: o } => {
                let old_pc = self.pc;
                match test {
                    RegOrInt::Int(i) => {
                        if i != 0 {
                            self.pc = ((self.pc as i32) + o) as u32;
                            if self.trace {
                                println!("Integer {} is not zero, shifting program counter {} by \
                                          {} to {}",
                                         i,
                                         old_pc,
                                         o,
                                         self.pc);
                            }
                        } else {
                            self.pc += 1;
                            if self.trace {
                                println!("Not jumping");
                            }
                        }
                    }
                    RegOrInt::Reg(register) => {
                        if self.value_of(register) != 0 {
                            self.pc = ((self.pc as i32) + o) as u32;
                            if self.trace {
                                println!("Register {:?} is not zero, shifting program counter {} \
                                          by {} to {}",
                                         register,
                                         old_pc,
                                         o,
                                         self.pc);
                            }
                        } else {
                            self.pc += 1;
                            if self.trace {
                                println!("Register {:?} is zero. Not jumping.", register);
                            }
                        }
                    }
                }
            }
        }

        return true;
    }

    pub fn run(&mut self) {
        let mut run = true;
        while run {
            run = self.run_next();
        }
    }
}

#[test]
fn test_interpret_copy() {
    let program = vec![Instruction::Copy {
                           from: RegOrInt::Int(1),
                           to: Register::A,
                       },
                       Instruction::Copy {
                           from: RegOrInt::Reg(Register::A),
                           to: Register::B,
                       }];
    let mut i = Interpreter::new(program);
    assert_eq!(i.value_of(Register::A), 0);
    assert_eq!(i.value_of(Register::B), 0);
    i.run_next();
    assert_eq!(i.value_of(Register::A), 1);
    assert_eq!(i.value_of(Register::B), 0);
    i.run_next();
    assert_eq!(i.value_of(Register::A), 1);
    assert_eq!(i.value_of(Register::B), 1);
}

#[test]
fn test_inc_dec() {
    let program = vec![Instruction::Copy {
                           from: RegOrInt::Int(2),
                           to: Register::A,
                       },
                       Instruction::Inc(Register::A),
                       Instruction::Dec(Register::A)];
    let mut i = Interpreter::new(program);
    assert_eq!(i.value_of(Register::A), 0);
    i.run_next();
    assert_eq!(i.value_of(Register::A), 2);
    i.run_next();
    assert_eq!(i.value_of(Register::A), 3);
    i.run_next();
    assert_eq!(i.value_of(Register::A), 2);
}

#[test]
fn test_jump() {
    let program = vec![Instruction::Copy {
                           from: RegOrInt::Int(2),
                           to: Register::A,
                       },
                       Instruction::Dec(Register::A),
                       Instruction::Jump {
                           test: RegOrInt::Reg(Register::A),
                           offset: -1,
                       },
                       Instruction::Copy {
                           from: RegOrInt::Int(3),
                           to: Register::B,
                       }];

    let mut i = Interpreter::new(program);
    let mut run = true;
    while run {
        run = i.run_next();
    }
    assert_eq!(i.value_of(Register::A), 0);
    assert_eq!(i.value_of(Register::B), 3);
}

#[test]
fn test_jump_int() {
    let program = vec![Instruction::Copy {
                           from: RegOrInt::Int(2),
                           to: Register::A,
                       },
                       Instruction::Dec(Register::A),
                       Instruction::Jump {
                           test: RegOrInt::Int(0),
                           offset: -1,
                       },
                       Instruction::Copy {
                           from: RegOrInt::Int(3),
                           to: Register::B,
                       }];

    let mut i = Interpreter::new(program);
    let mut run = true;
    while run {
        run = i.run_next();
    }
    assert_eq!(i.value_of(Register::A), 1);
    assert_eq!(i.value_of(Register::B), 3);
}
