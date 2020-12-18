use crate::{
    dayerror::DayError,
    grid3d::{Grid3D, HashMapGrid3D},
    grid4d::{Grid4D, HashMapGrid4D},
    point3d::Point3D,
    point4d::Point4D,
};

pub fn part1() -> Result<String, DayError> {
    Ok(format!(
        "There are {} active elements",
        run_part1(include_str!("input.txt"))?
    ))
}

pub fn part2() -> Result<String, DayError> {
    Ok(format!(
        "There are {} active elements",
        run_part2(include_str!("input.txt"))?
    ))
}

fn run_part1(input: &str) -> Result<usize, DayError> {
    let mut grid = initial_state_3d(input)?;
    for _ in 0..6 {
        grid = iterate_part1(&grid);
    }
    Ok(grid.count_set_elements_equal_to(&true))
}

fn run_part2(input: &str) -> Result<usize, DayError> {
    let mut grid = initial_state_4d(input)?;
    for _ in 0..6 {
        grid = iterate_part2(&grid);
    }
    Ok(grid.count_set_elements_equal_to(&true))
}

fn initial_state_3d(input: &str) -> Result<HashMapGrid3D<bool>, DayError> {
    let z = 0;
    let mut grid = HashMapGrid3D::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                grid.set(Point3D::new(x as i64, y as i64, z), true);
            }
        }
    }
    Ok(grid)
}

fn initial_state_4d(input: &str) -> Result<HashMapGrid4D<bool>, DayError> {
    let z = 0;
    let w = 0;
    let mut grid = HashMapGrid4D::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                grid.set(Point4D::new(x as i64, y as i64, z, w), true);
            }
        }
    }
    Ok(grid)
}

fn surrounding_active_3d(grid: &impl Grid3D<bool>, p: &Point3D) -> usize {
    grid.get_neighbours(p, &false)
        .into_iter()
        .filter(|c| **c)
        .count()
}

fn surrounding_active_4d(grid: &impl Grid4D<bool>, p: &Point4D) -> usize {
    grid.get_neighbours(p, &false)
        .into_iter()
        .filter(|c| **c)
        .count()
}

fn iterate_part1<G: Grid3D<bool> + Clone>(grid: &G) -> G {
    grid.simultaneous_apply(|point| match grid.get(point) {
        Some(&true) => match surrounding_active_3d(grid, point) {
            2 | 3 => true,
            _ => false,
        },
        _ => surrounding_active_3d(grid, point) == 3,
    })
}

fn iterate_part2<G: Grid4D<bool> + Clone>(grid: &G) -> G {
    grid.simultaneous_apply(|point| match grid.get(point) {
        Some(&true) => match surrounding_active_4d(grid, point) {
            2 | 3 => true,
            _ => false,
        },
        _ => surrounding_active_4d(grid, point) == 3,
    })
}

#[test]
fn test_part1_iterate() {
    let initial = initial_state_3d(
        ".#.
..#
###",
    )
    .unwrap();
    assert_eq!(initial.count_set_elements_equal_to(&true), 5);

    let iterated = iterate_part1(&initial);
    assert_eq!(
        iterated.count_set_elements_equal_to(&true),
        11,
        "count after first iteration should be 11 active"
    );
    let iterated = iterate_part1(&iterated);
    assert_eq!(
        iterated.count_set_elements_equal_to(&true),
        21,
        "count after second iteration should be 21 active"
    );
    let iterated = iterate_part1(&iterated);
    assert_eq!(
        iterated.count_set_elements_equal_to(&true),
        38,
        "count after third iteration should be 38 active"
    );
}

#[test]
fn test_parse_input() {
    let grid = initial_state_3d(
        ".#.
..#
###",
    )
    .unwrap();

    assert_eq!(grid.get(&Point3D::new(0, 0, 0)).unwrap_or(&false), &false);
    assert_eq!(grid.get(&Point3D::new(1, 0, 0)).unwrap_or(&false), &true);
    assert_eq!(grid.get(&Point3D::new(2, 0, 0)).unwrap_or(&false), &false);
    assert_eq!(grid.get(&Point3D::new(0, 1, 0)).unwrap_or(&false), &false);
    assert_eq!(grid.get(&Point3D::new(1, 1, 0)).unwrap_or(&false), &false);
    assert_eq!(grid.get(&Point3D::new(2, 1, 0)).unwrap_or(&false), &true);
    assert_eq!(grid.get(&Point3D::new(0, 2, 0)).unwrap_or(&false), &true);
    assert_eq!(grid.get(&Point3D::new(1, 2, 0)).unwrap_or(&false), &true);
    assert_eq!(grid.get(&Point3D::new(2, 2, 0)).unwrap_or(&false), &true);
}

#[test]
fn test_part1_sample() {
    assert_eq!(
        run_part1(
            ".#.
..#
###"
        )
        .unwrap(),
        112
    );
}

#[test]
fn test_part2_sample() {
    assert_eq!(
        run_part2(
            ".#.
..#
###"
        )
        .unwrap(),
        848
    );
}
