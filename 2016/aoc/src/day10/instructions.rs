
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Target {
    Output(u32),
    Bot(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Give { value: u32, to: u32 },
    Decide { bot: u32, low: Target, high: Target },
}


impl Instruction {
    pub fn give(value: u32, to: u32) -> Instruction {
        Instruction::Give {
            value: value,
            to: to,
        }
    }

    pub fn decide(bot: u32, low: Target, high: Target) -> Instruction {
        Instruction::Decide {
            bot: bot,
            low: low,
            high: high,
        }
    }
}
