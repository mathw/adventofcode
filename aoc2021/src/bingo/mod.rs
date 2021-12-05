use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone)]
pub struct Board {
    numbers: [u8; 25],
    marks: [bool; 25],
}

impl Board {
    pub fn new(numbers: [u8; 25]) -> Board {
        Board {
            numbers,
            marks: [false; 25],
        }
    }

    fn index_of(x: usize, y: usize) -> Option<usize> {
        let index = 5 * x + y;
        if index > 24 {
            None
        } else {
            Some(index)
        }
    }

    pub fn number_at(&self, x: usize, y: usize) -> Option<u8> {
        Some(self.numbers[Board::index_of(x, y)?])
    }

    pub fn is_marked(&self, x: usize, y: usize) -> Option<bool> {
        Some(self.marks[Board::index_of(x, y)?])
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        if let Some(index) = Board::index_of(x, y) {
            self.marks[index] = true;
        }
    }

    pub fn check_state(&self) -> BoardState {
        // check rows
        for row in 0..5 {
            if (0..5).all(|col| {
                self.is_marked(row, col)
                    .expect("Shouldn't overflow in internal code (check_state rows)")
            }) {
                return BoardState::WinRow(row);
            }
        }
        // check columns
        for col in 0..5 {
            if (0..5).all(|row| {
                self.is_marked(row, col)
                    .expect("shouldn't overflow (check_state cols)")
            }) {
                return BoardState::WinColumn(col);
            }
        }
        BoardState::NotWin
    }

    pub fn is_win(&self) -> bool {
        self.check_state() != BoardState::NotWin
    }

    pub fn all_unmarked_numbers(&self) -> Vec<u8> {
        (0..25)
            .filter_map(|i| {
                if self.marks[i] {
                    None
                } else {
                    Some(self.numbers[i])
                }
            })
            .collect()
    }

    pub fn mark_number(&mut self, num: u8) -> bool {
        // assumes numbers are unique
        let index = self.numbers.iter().position(|n| *n == num);
        match index {
            None => false,
            Some(i) => {
                self.marks[i] = true;
                true
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardState {
    NotWin,
    WinRow(usize),
    WinColumn(usize),
}

impl FromStr for Board {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = input.lines().collect();
        if rows.len() != 5 {
            return Err(format!("Input does not have five rows"));
        }

        let mut board_numbers: [u8; 25] = [0; 25];
        let mut index = 0;
        for row in rows {
            let numbers: Vec<_> = row.split_whitespace().collect();
            if numbers.len() != 5 {
                return Err(format!(
                    "Input does not have five columns on this row: {}",
                    row
                ));
            }
            for number in numbers {
                if let Ok(n) = u8::from_str(number) {
                    board_numbers[index] = n;
                } else {
                    return Err(format!("Unable to parse supposed number {}", number));
                }
                index += 1;
            }
        }

        if index != 25 {
            return Err(format!("Didn't get 25 numbers, got {}", index + 1));
        }

        let board = Board::new(board_numbers);

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..5 {
            for col in 0..5 {
                write!(f, "{} ", self.number_at(row, col).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
