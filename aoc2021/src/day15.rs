use crate::{
    common::grid::Grid,
    day::{DayResult, PartResult},
};
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let part1 = part1(include_str!("inputs/day15.txt"))?;
    Ok(DayResult::new(
        PartResult::Success(format!("{} is the lowest risk path", part1)),
        PartResult::NotImplemented,
    ))
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let grid = parse_input_grid(input)?;
    Ok(djikstra(&grid))
}

fn parse_input_grid(input: &str) -> Result<Grid<u8>, Box<dyn Error>> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();

    let mut grid = Grid::new(width, lines.len());

    for (y, row) in lines.into_iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid.set(x, y, format!("{}", c).parse::<u8>()?);
        }
    }

    Ok(grid)
}

fn djikstra(caves_grid: &Grid<u8>) -> usize {
    println!("{}\n-----------------------", caves_grid);
    let mut current_node = (0, 0);
    let destination_node = (caves_grid.width() - 1, caves_grid.height() - 1);
    let mut node_distances: Grid<usize> =
        Grid::new_with_value(caves_grid.width(), caves_grid.height(), usize::MAX);
    let mut visited_grid: Grid<bool> = Grid::new(caves_grid.width(), caves_grid.height());
    node_distances.set(current_node.0, current_node.1, 0);

    loop {
        // termination condition
        if current_node == destination_node {
            println!("{}", node_distances);
            return *node_distances
                .get(destination_node.0, destination_node.1)
                .expect("This should always find an answer");
        }

        let current_distance = *node_distances.get(current_node.0, current_node.1).unwrap();
        let is_problem_node = current_node == (1, 3);
        if is_problem_node {
            println!(
                "Evaluating node {:?} current distance is {}\nDistance grid state is\n{}\n",
                current_node, current_distance, node_distances
            );
        }

        for neighbour in caves_grid.surrounding_coords_no_diagonals(current_node.0, current_node.1)
        {
            if is_problem_node {
                println!(
                    "Evaluating neighbour {:?} with cost {}...",
                    neighbour,
                    caves_grid.get(neighbour.0, neighbour.1).unwrap()
                );
            }

            let neighbour_distance_from_here =
                current_distance + *caves_grid.get(neighbour.0, neighbour.1).unwrap() as usize;

            if is_problem_node {
                println!("Distance is {}", neighbour_distance_from_here);
            }

            if neighbour_distance_from_here < *node_distances.get(neighbour.0, neighbour.1).unwrap()
            {
                if is_problem_node {
                    println!("Distance replaced as it is shorter");
                }
                node_distances.set(neighbour.0, neighbour.1, neighbour_distance_from_here);
            }
        }

        visited_grid.set(current_node.0, current_node.1, true);

        let smallest_unvisited_neighbour = caves_grid
            .surrounding_coords_no_diagonals(current_node.0, current_node.1)
            .filter(|(nx, ny)| !*visited_grid.get(*nx, *ny).unwrap())
            .min_by_key(|(nx, ny)| visited_grid.get(*nx, *ny).unwrap())
            .expect("Should have found a smallest unvisited node");

        current_node = smallest_unvisited_neighbour;
    }
}

impl std::fmt::Display for Grid<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let val = self.get(x, y).unwrap();
                write!(
                    f,
                    " {} ",
                    if *val == usize::MAX {
                        format!("--")
                    } else {
                        format!("{:02}", val)
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, " {} ", self.get(x, y).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn test_part1_sample() {
    let caves = parse_input_grid(include_str!("inputs/samples/day15.txt")).unwrap();
    let total_risk = djikstra(&caves);
    assert_eq!(total_risk, 40);
}
