use super::maze::{Maze, Cell};

pub fn parse(input: &str) -> Maze {
    let lines = input.lines().collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    let mut maze = Maze::new(width, height);

    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let cell = match c {
                '#' => Cell::Wall,
                '.' => Cell::Open,
                '0' => Cell::Goal(0),
                '1' => Cell::Goal(1),
                '2' => Cell::Goal(2),
                '3' => Cell::Goal(3),
                '4' => Cell::Goal(4),
                '5' => Cell::Goal(5),
                '6' => Cell::Goal(6),
                '7' => Cell::Goal(7),
                '8' => Cell::Goal(8),
                '9' => Cell::Goal(9),
                _ => Cell::Wall,
            };

            maze.mark(x, y, cell);
        }
    }

    maze
}
