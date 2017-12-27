mod grid;
mod rule;
mod rulebook;

use std::str::FromStr;
use self::rulebook::Rulebook;
use self::grid::Grid;

static STARTING_IMAGE: &str = ".#./..#/###";

pub fn go() {
    let input = include_str!("input.txt");

    let rulebook = Rulebook::from_str(input).expect("Rulebook failed to parse");
    let start = Grid::from_str(STARTING_IMAGE).expect("Starting image failed to parse");

    let mut current = Some(start);

    let mut after5 = None;
    let mut after18 = None;

    for iteration in 0..18 {
        println!("Iteration {}", iteration + 1);

        current = rulebook.apply_to(&current.unwrap());
        if current.is_none() {
            panic!("Iteration {}: no output!", iteration);
        }
        if iteration == 4 {
            after5 = current.clone();
        }
        if iteration == 17 {
            after18 = current.clone();
        }
    }

    println!(
        "{} pixels lit after 5, {} pixels lit after 18",
        after5.unwrap().count_lit(),
        after18.unwrap().count_lit()
    );
}
