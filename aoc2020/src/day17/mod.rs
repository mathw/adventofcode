use crate::{
    dayerror::DayError,
    grid3d::{Grid3D, HashMapGrid3D},
    point3d::Point3D,
};

pub fn part1() -> Result<String, DayError> {
    Ok(format!(
        "There are {} active elements",
        run_part1(include_str!("input.txt"))?
    ))
}

fn run_part1(input: &str) -> Result<usize, DayError> {
    let mut grid = initial_state(input)?;
    for _ in 0..6 {
        grid = iterate_part1(&grid);
    }
    Ok(grid.count_set_elements_equal_to(&true))
}

fn initial_state(input: &str) -> Result<HashMapGrid3D<bool>, DayError> {
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

fn surrounding_active(grid: &impl Grid3D<bool>, p: &Point3D) -> usize {
    grid.get_neighbours(p, &false)
        .into_iter()
        .filter(|c| **c)
        .count()
}

fn iterate_part1<G: Grid3D<bool> + Clone>(grid: &G) -> G {
    grid.simultaneous_apply(|point| match grid.get(point) {
        Some(&true) => match surrounding_active(grid, point) {
            2 | 3 => true,
            _ => false,
        },
        _ => surrounding_active(grid, point) == 3,
    })
}

#[test]
fn test_part1_iterate() {
    let initial = initial_state(
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
    let grid = initial_state(
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
    let grid = initial_state(
        ".#.
..#
###",
    )
    .unwrap();

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
