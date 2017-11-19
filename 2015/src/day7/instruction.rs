#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Signal { value: u16, target: String },
    And {
        a: String,
        b: String,
        target: String,
    },
    Or {
        a: String,
        b: String,
        target: String,
    },
    Shift {
        input: String,
        distance: i32,
        target: String,
    },
    Not { input: String, target: String },
}