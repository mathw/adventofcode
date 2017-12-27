use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    pub lines: Vec<Vec<bool>>,
}

impl Grid {
    /// Test if a grid is a match over mirroring and rotation for another grid.
    pub fn is_match(&self, other: &Grid) -> bool {
        self.clone().into_iter().any(|s| s == *other)
    }

    pub fn size(&self) -> usize {
        self.lines.len()
    }

    pub fn as_threes(&self) -> Option<Vec<Vec<Grid>>> {
        self.get_squares(3)
    }

    pub fn as_twos(&self) -> Option<Vec<Vec<Grid>>> {
        self.get_squares(2)
    }

    pub fn count_lit(&self) -> usize {
        self.lines
            .iter()
            .map(|l| l.iter().filter(|x| **x).count())
            .sum()
    }

    fn get_squares(&self, size: usize) -> Option<Vec<Vec<Grid>>> {
        if self.size() % size != 0 {
            return None;
        }

        let mut result = Vec::new();

        let count = self.size() / size;
        for ycount in 0..count {
            let mut result_line = Vec::new();
            for xcount in 0..count {
                let mut output = Vec::new();
                for y in 0..size {
                    let mut line = Vec::new();
                    for x in 0..size {
                        let required_x = x + (xcount * size);
                        let required_y = y + (ycount * size);

                        line.push(self.get(required_x, required_y).expect(&format!(
                            "Getting ({}, {}) should work",
                            required_x,
                            required_y
                        )));
                    }
                    output.push(line);
                }
                result_line.push(Grid { lines: output });
            }
            result.push(result_line);
        }

        Some(result)
    }

    pub fn lines(&self) -> &Vec<Vec<bool>> {
        &self.lines
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.lines.get(y).and_then(|line| line.get(x)).map(|a| *a)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.lines
            .iter()
            .map(|line| {
                line.iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("/");
        write!(f, "{}", s)
    }
}

pub fn merge_grids(grids: &Vec<Vec<Grid>>) -> Option<Grid> {
    if grids.len() == 0 {
        return None;
    }

    if grids.len() != grids[0].len() {
        // not square
        return None;
    }

    let size = grids[0][0].lines.len();

    let result = grids
        .iter()
        .flat_map(|squares_line| {
            (0..size)
                .map(|a| {
                    squares_line
                        .iter()
                        .flat_map(|square| square.lines[a].clone())
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(Grid { lines: result })
}


/// Rotate a square grid 90 degrees
fn rotate_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let size = grid.len();
    let mut output = Vec::new();
    for x in 0..size {
        let mut line = grid.iter().map(|l| l[x]).collect::<Vec<_>>();
        line.reverse();
        output.push(line);
    }

    output
}

fn flip_grid_vertical(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut output = grid.clone();
    output.reverse();
    output
}

fn flip_grid_horizontal(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut output = Vec::new();
    for mut line in grid.clone() {
        line.reverse();
        output.push(line);
    }

    output
}

enum Permutation {
    Original,
    Rotate1,
    Rotate2,
    Rotate3,
    FlipHorizontal,
    FlipHorizontalRotate1,
    FlipHorizontalRotate2,
    FlipHorizontalRotate3,
    FlipVertical,
    FlipVerticalRotate1,
    FlipVerticalRotate2,
    FlipVerticalRotate3,
    Complete,
}

pub struct GridPermutations {
    current: Vec<Vec<bool>>,
    permutation: Permutation,
    original: Vec<Vec<bool>>,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = Grid;
    type IntoIter = GridPermutations;

    fn into_iter(self) -> GridPermutations {
        GridPermutations {
            current: self.lines.clone(),
            permutation: Permutation::Original,
            original: self.lines.clone(),
        }
    }
}

impl Iterator for GridPermutations {
    type Item = Grid;

    fn next(&mut self) -> Option<Grid> {
        match self.permutation {
            Permutation::Original => {
                self.permutation = Permutation::Rotate1;
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::Rotate1 => {
                self.permutation = Permutation::Rotate2;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::Rotate2 => {
                self.permutation = Permutation::Rotate3;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::Rotate3 => {
                self.permutation = Permutation::FlipHorizontal;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipHorizontal => {
                self.permutation = Permutation::FlipHorizontalRotate1;
                self.current = flip_grid_horizontal(&self.original);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipHorizontalRotate1 => {
                self.permutation = Permutation::FlipHorizontalRotate2;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipHorizontalRotate2 => {
                self.permutation = Permutation::FlipHorizontalRotate3;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipHorizontalRotate3 => {
                self.permutation = Permutation::FlipVertical;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipVertical => {
                self.permutation = Permutation::FlipVerticalRotate1;
                self.current = flip_grid_vertical(&self.original);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipVerticalRotate1 => {
                self.permutation = Permutation::FlipVerticalRotate2;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipVerticalRotate2 => {
                self.permutation = Permutation::FlipVerticalRotate3;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::FlipVerticalRotate3 => {
                self.permutation = Permutation::Complete;
                self.current = rotate_grid(&self.current);
                Some(Grid {
                    lines: self.current.clone(),
                })
            }
            Permutation::Complete => None,
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Grid, ()> {
        let pattern_lines = pattern_lines(s);

        if is_square(&pattern_lines) {
            Ok(Grid {
                lines: pattern_lines,
            })
        } else {
            Err(())
        }
    }
}

fn is_square<T>(x: &Vec<Vec<T>>) -> bool {
    let count = x.len();
    x.iter().all(|y| y.len() == count)
}

fn pattern_lines(input: &str) -> Vec<Vec<bool>> {
    input
        .split("/")
        .filter_map(|l| match l.trim() {
            "" => None,
            x => Some(
                x.chars()
                    .map(|c| match c {
                        '#' => true,
                        _ => false,
                    })
                    .collect::<Vec<bool>>(),
            ),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_rotate() {
        let start = vec![vec![true, false], vec![false, false]];

        let one = rotate_grid(&start);
        let two = rotate_grid(&one);
        let three = rotate_grid(&two);
        let four = rotate_grid(&three);

        assert_eq!(one, vec![vec![false, true], vec![false, false]]);
        assert_eq!(two, vec![vec![false, false], vec![false, true]]);
        assert_eq!(three, vec![vec![false, false], vec![true, false]]);
        assert_eq!(four, start);
    }

    #[test]
    fn can_flip_vertical() {
        let start = vec![vec![true, false], vec![false, false]];
        assert_eq!(
            flip_grid_vertical(&start),
            vec![vec![false, false], vec![true, false]]
        );
    }

    #[test]
    fn can_flip_horizontal() {
        let start = vec![vec![true, false], vec![false, false]];
        assert_eq!(
            flip_grid_horizontal(&start),
            vec![vec![false, true], vec![false, false]]
        );
    }

    #[test]
    fn matches_sample() {
        let sample = Grid::from_str(".#./..#/###").unwrap();
        let match1 = Grid::from_str(".#./..#/###").unwrap();
        let match2 = Grid::from_str(".#./#../###").unwrap();
        let match3 = Grid::from_str("#../#.#/##.").unwrap();
        let match4 = Grid::from_str("###/..#/.#.").unwrap();

        assert!(sample.is_match(&match1), "First sample matches");
        assert!(sample.is_match(&match2), "Second sample matches");
        assert!(sample.is_match(&match3), "Third sample matches");
        assert!(sample.is_match(&match4), "Fourth sample matches");
    }

    #[test]
    fn test_merge_grids() {
        let input1 = Grid::from_str("../..").unwrap();
        let input2 = Grid::from_str("##/##").unwrap();
        let input3 = Grid::from_str("#./#.").unwrap();
        let input4 = Grid::from_str(".#/.#").unwrap();

        let input_grids = vec![vec![input1, input2], vec![input3, input4]];

        let expected = "..##/..##/#..#/#..#";

        assert_eq!(
            format!("{}", merge_grids(&input_grids).unwrap()),
            expected.to_owned()
        );
    }
}
