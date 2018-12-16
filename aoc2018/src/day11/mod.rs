use crate::day::Day;
use itertools::unfold;
use std::collections::HashMap;
use std::i32;
use std::sync::mpsc::Sender;

pub struct Day11 {
    serial: usize,
}

impl Default for Day11 {
    fn default() -> Self {
        Day11 { serial: 9005 }
    }
}

impl Day for Day11 {
    fn part1(&mut self, sender: &Sender<String>) {
        let grid = Grid::new(300, 300, self.serial);
        let (x, y, _) = grid.highest_power_region(3, 3);

        sender.send(format!("{},{}", x, y)).unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let grid = Grid::new(300, 300, self.serial);
        let mut largest = HashMap::new();
        for size in 1..=300 {
            largest.insert(size, grid.highest_power_region(size, size));
        }

        let mut highest_power = i32::MIN;
        let mut highest = (0, 0, 0);
        for (size, (x, y, power)) in largest {
            if power > highest_power {
                highest_power = power;
                highest = (x, y, size);
            }
        }

        sender
            .send(format!("{},{},{}", highest.0, highest.1, highest.2))
            .unwrap();
    }
}

struct Grid {
    cells: Vec<i32>,
    width: usize,
    height: usize,
    serial_number: usize,
}

impl Grid {
    fn new(width: usize, height: usize, serial_number: usize) -> Grid {
        let mut grid = Grid {
            cells: Vec::with_capacity(width * height),
            width,
            height,
            serial_number,
        };
        grid.calculate_all_fuel_cells();
        grid
    }

    fn calculate_all_fuel_cells(&mut self) {
        self.cells.clear();
        for index in 0..self.width * self.height {
            let (x, y) = self.coords_from_index(index);
            self.cells.push(self.power_level_for_cell(x, y));
        }
    }

    fn total_cluster_power(
        &self,
        x: usize,
        y: usize,
        cluster_width: usize,
        cluster_height: usize,
    ) -> i32 {
        let mut result: i32 = 0;

        for cy in 0..cluster_height {
            let start_x = (x - 1) + ((y - 1) * self.width) + (cy * self.width);
            let sum = self.cells[start_x..start_x + cluster_width]
                .iter()
                .sum::<i32>();
            result += sum;
        }

        result
    }

    fn highest_power_region(&self, width: usize, height: usize) -> (usize, usize, i32) {
        let mut highest_power_so_far = i32::MIN;
        let mut highest_location = (0, 0);
        for (x, y) in all_boxes_in_grid(self.width, self.height, width, height) {
            let power = self.total_cluster_power(x, y, width, height);
            if highest_power_so_far < power {
                highest_power_so_far = power;
                highest_location = (x, y);
            }
        }

        (highest_location.0, highest_location.1, highest_power_so_far)
    }

    fn coords_from_index(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;

        (x + 1, y + 1)
    }

    fn power_level_for_cell(&self, x: usize, y: usize) -> i32 {
        let rack_id = x + 10;
        let power_level = rack_id * y;
        let power_level = power_level + self.serial_number;
        let power_level = power_level * rack_id;
        let power_level = (power_level / 100) % 10;

        power_level as i32 - 5
    }
}

fn all_boxes_in_grid(
    width: usize,
    height: usize,
    box_width: usize,
    box_height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    unfold((1, 1), move |(x, y)| {
        if *x + box_width > width + 1 {
            *x = 1;
            *y += 1;
        }
        if *y + box_height > height + 1 {
            None
        } else {
            let result = Some((*x, *y));
            *x += 1;
            result
        }
    })
}

#[test]
fn test_power_level_example() {
    let grid = Grid::new(300, 300, 8);
    let level = grid.power_level_for_cell(3, 5);

    assert_eq!(level, 4);
}

#[test]
fn test_power_level_example_two() {
    let grid = Grid::new(300, 300, 57);
    let level = grid.power_level_for_cell(122, 79);

    assert_eq!(level, -5);
}
#[test]
fn test_power_level_example_three() {
    let grid = Grid::new(300, 300, 39);
    let level = grid.power_level_for_cell(217, 196);

    assert_eq!(level, 0);
}
#[test]
fn test_power_level_example_four() {
    let grid = Grid::new(300, 300, 71);
    let level = grid.power_level_for_cell(101, 153);

    assert_eq!(level, 4);
}

#[test]
fn test_cluster_power() {
    let grid = Grid::new(300, 300, 18);
    let power = grid.total_cluster_power(33, 45, 3, 3);
    assert_eq!(power, 29);
}

#[test]
fn test_find_region() {
    let grid = Grid::new(300, 300, 18);
    let (x, y, power) = grid.highest_power_region(3, 3);
    assert_eq!(x, 33);
    assert_eq!(y, 45);
    assert_eq!(power, 29);
}

#[test]
fn all_boxes_300() {
    let boxes = all_boxes_in_grid(300, 300, 300, 300).collect::<Vec<_>>();
    assert_eq!(boxes, vec![(1, 1)]);
}

#[test]
fn all_boxes_299() {
    let boxes = all_boxes_in_grid(300, 300, 299, 299).collect::<Vec<_>>();
    assert_eq!(boxes, vec![(1, 1), (2, 1), (1, 2), (2, 2)]);
}

#[test]
fn all_boxes_298() {
    let boxes = all_boxes_in_grid(300, 300, 298, 298).collect::<Vec<_>>();
    assert_eq!(
        boxes,
        vec![
            (1, 1),
            (2, 1),
            (3, 1),
            (1, 2),
            (2, 2),
            (3, 2),
            (1, 3),
            (2, 3),
            (3, 3)
        ]
    );
}
