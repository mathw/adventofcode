use crate::common::position::Position;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = String;
    fn from_str(l: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let parts = l.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!(
                "Command string \"{}\" does not parse into two parts",
                l
            ));
        }
        let num = u32::from_str(parts[1]).map_err(|_| format!("\"{}\" is not a u32", parts[1]))?;
        match parts[0] {
            "forward" => Ok(Command::Forward(num)),
            "down" => Ok(Command::Down(num)),
            "up" => Ok(Command::Up(num)),
            x => Err(format!("\"{}\" is not a recognised command", x)),
        }
    }
}

impl Command {
    pub fn execute_commands(start: Position, commands: impl Iterator<Item = Command>) -> Position {
        let mut current_pos = start;

        for command in commands {
            match command {
                Command::Forward(distance) => current_pos = current_pos.forward(distance),
                Command::Down(distance) => current_pos = current_pos.down(distance),
                Command::Up(distance) => current_pos = current_pos.up(distance),
            }
        }

        current_pos
    }

    pub fn execute_commands_with_aim(
        start: Position,
        commands: impl Iterator<Item = Command>,
    ) -> Position {
        let mut current_pos = start;
        let mut current_aim = 0;

        for command in commands {
            match command {
                Command::Forward(distance) => {
                    current_pos = current_pos.forward_with_aim(current_aim, distance)
                }
                Command::Down(distance) => current_aim += distance as i64,
                Command::Up(distance) => current_aim -= distance as i64,
            }
        }

        current_pos
    }
}
