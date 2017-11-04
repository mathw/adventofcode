#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Wall,
    Open,
    Goal(u8),
}

#[derive(Debug)]
pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            width: width,
            height: height,
            cells: vec![Cell::Wall; width * height],
        }
    }

    fn index_of(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn mark(&mut self, x: usize, y: usize, new_value: Cell) {
        let index = self.index_of(x, y);
        let c = self.cells.get_mut(index).unwrap();
        *c = new_value;
    }

    pub fn render(&self) -> String {
        let mut r = String::new();

        for l in (0..self.height).map(|y| {
            (0..self.width)
                .map(|x| {
                    match self.cells[self.index_of(x, y)] {
                        Cell::Wall => "#".to_owned(),
                        Cell::Open => ".".to_owned(),
                        Cell::Goal(i) => format!("{}", i),
                    }
                })
                .collect::<String>()
        }) {
            r.push_str(&l);
            r.push_str("\n")
        }

        r
    }
}
