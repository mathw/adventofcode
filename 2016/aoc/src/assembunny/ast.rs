#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegOrInt {
    Reg(Register),
    Int(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Copy { from: RegOrInt, to: Register },
    Inc(Register),
    Dec(Register),
    Jump { test: RegOrInt, offset: RegOrInt },
    Toggle(RegOrInt),
    InvalidInc(i32),
    InvalidCopy { test: RegOrInt, offset: i32 },
    InvalidDec(i32),
}
