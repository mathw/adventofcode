#[derive(Debug, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Step {
    pub turn: Turn,
    pub blocks: u32,
}

impl Step {
    pub fn new(turn: Turn, blocks: u32) -> Step {
        Step {
            turn: turn,
            blocks: blocks,
        }
    }
}
