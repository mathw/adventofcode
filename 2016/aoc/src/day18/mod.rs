use std::fmt;
use std::fmt::Display;

pub fn do_day18() {
    let input = ".^^..^...^..^^.^^^.^^^.^^^^^^.^.^^^^.^^.^^^^^^.^...^......^...^^^..^^^.....\
                 ^^^^^^^^^....^^...^^^^..^";

    let grid = make_grid(&Tile::from_str(input), 40);

    println!("Safe tiles (40 rows): {}", safe_tiles_in_grid(&grid));

    let biggrid = make_grid(&Tile::from_str(input), 400000);
    println!("Safe tiles (400000 rows): {}", safe_tiles_in_grid(&biggrid));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Safe,
    Trap,
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &Tile::Safe => '.',
                   &Tile::Trap => '^',
               })
    }
}


impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Safe),
            '^' => Some(Tile::Trap),
            _ => None,
        }
    }

    fn is_safe(&self) -> bool {
        match self {
            &Tile::Safe => true,
            &Tile::Trap => false,
        }
    }

    fn from_row(row: &Vec<Tile>) -> Vec<Tile> {
        let mut result = Vec::new();
        let parent = wrap_vec(row, &Tile::Safe);

        for i in 0..parent.len() - 2 {
            let left = parent[i];
            let centre = parent[i + 1];
            let right = parent[i + 2];

            result.push(match (left, centre, right) {
                (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
                (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            });
        }

        result
    }

    fn from_str(src: &str) -> Vec<Tile> {
        src.chars().filter_map(|c| Tile::from_char(c)).collect()
    }
}

fn make_grid(first_row: &Vec<Tile>, rows: usize) -> Vec<Vec<Tile>> {
    let mut grid = Vec::new();
    grid.push(first_row.clone());

    for i in 1..rows {
        let new_row = Tile::from_row(&grid[i - 1]);
        grid.push(new_row);
    }

    grid
}

fn wrap_vec<T>(src: &Vec<T>, pad: &T) -> Vec<T>
    where T: Clone
{
    let mut result = Vec::new();
    result.push(pad.clone());
    result.append(&mut src.clone());
    result.push(pad.clone());
    result
}

fn safe_tiles_in_row(row: &Vec<Tile>) -> usize {
    row.into_iter().filter(|t| t.is_safe()).count()
}

fn safe_tiles_in_grid(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter().map(|r| safe_tiles_in_row(r)).sum()
}

#[test]
fn test_wrap_vec() {
    let src = vec!['a', 'b'];
    let pad = 'c';
    let res = wrap_vec(&src, &pad);

    assert_eq!(res, vec!['c', 'a', 'b', 'c']);
}

#[test]
fn test_from_row() {
    let parent = vec![Tile::Safe, Tile::Safe, Tile::Trap, Tile::Trap, Tile::Safe];
    let row = Tile::from_row(&parent);

    assert_eq!(row,
               vec![Tile::Safe, Tile::Trap, Tile::Trap, Tile::Trap, Tile::Trap]);

    let row2 = Tile::from_row(&row);

    assert_eq!(row2,
               vec![Tile::Trap, Tile::Trap, Tile::Safe, Tile::Safe, Tile::Trap]);
}

#[test]
fn test_from_str() {
    let s = "..^^.";
    let row = Tile::from_str(s);

    assert_eq!(row,
               vec![Tile::Safe, Tile::Safe, Tile::Trap, Tile::Trap, Tile::Safe]);
}
