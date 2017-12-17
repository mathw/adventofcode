use util::knot_hash;
use util::timed;
use std::collections::HashSet;

pub fn go() {
    let input = "hwlqcszp";
    let input = "flqrgnkx";

    let (result, time) = timed(|| count_used_in_grid(input));
    println!("[{}ms] {} squares used in the grid", time, result);

    let (regions, time) = timed(|| {
        let rows = (0..128).map(|r| row(input, r)).collect();
        let regions = create_regions_from_grid(&rows);
        count_unique_regions(&regions)
    });

    println!("[{}ms] {} regions", time, regions);
}

fn count_used_in_grid(input: &str) -> usize {
    (0..128)
        .map(|r| row(input, r).iter().filter(|&&x| x).count())
        .sum()
}

fn row(input: &str, row: usize) -> Vec<bool> {
    let input = format!("{}-{}", input, row);
    hash_as_bits(&knot_hash(input.chars().map(|c| c as u8)))
}

fn hash_as_bits(hash: &str) -> Vec<bool> {
    hash.chars().flat_map(|c| char_to_bits(c)).collect()
}

fn char_to_bits(c: char) -> Vec<bool> {
    match c {
        '0' => vec![false, false, false, false],
        '1' => vec![false, false, false, true],
        '2' => vec![false, false, true, false],
        '3' => vec![false, false, true, true],
        '4' => vec![false, true, false, false],
        '5' => vec![false, true, false, true],
        '6' => vec![false, true, true, false],
        '7' => vec![false, true, true, true],
        '8' => vec![true, false, false, false],
        '9' => vec![true, false, false, true],
        'a' => vec![true, false, true, false],
        'b' => vec![true, false, true, true],
        'c' => vec![true, true, false, false],
        'd' => vec![true, true, false, true],
        'e' => vec![true, true, true, false],
        'f' => vec![true, true, true, true],
        _ => panic!("Hash value out of bounds {}", c),
    }
}

fn render_grid(rows: &Vec<Vec<Region>>, grid_width: usize) {
    for row in rows.iter() {
        let mut current_position = 0;
        for region in row {
            while current_position < region.start_index {
                current_position += 1;
                print!(".. ");
            }
            for _ in 0..region.length {
                current_position += 1;
                print!("{:02} ", region.label);
            }
        }
        while current_position < grid_width {
            current_position += 1;
            print!(".. ");
        }
        print!("\n");
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Region {
    start_index: usize,
    length: usize,
    label: u16,
}

impl Region {
    fn new(start_index: usize, length: usize, label: u16) -> Region {
        Region {
            start_index: start_index,
            length: length,
            label: label,
        }
    }

    fn end_index(&self) -> usize {
        self.start_index + self.length
    }

    fn intersects_with(&self, other: &Region) -> bool {
        (self.start_index <= other.start_index && self.end_index() > other.start_index)
            || (self.start_index < other.end_index() && self.end_index() > other.start_index)
    }

    fn with_label(&self, label: u16) -> Region {
        Region {
            start_index: self.start_index,
            length: self.length,
            label: label,
        }
    }
}

fn unify_regions_to_label(rows: &mut Vec<Vec<Region>>, labels: &HashSet<u16>, new_label: u16) {
    for row in rows {
        for region in row {
            if labels.contains(&region.label) {
                region.label = new_label;
            }
        }
    }
}

fn count_unique_regions(grid: &Vec<Vec<Region>>) -> usize {
    let mut labels = HashSet::new();

    for row in grid {
        for region in row {
            labels.insert(region.label);
        }
    }

    labels.len()
}

fn create_regions_from_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<Region>> {
    let mut rows = Vec::new();
    let mut previous_row: Option<Vec<Region>> = None;
    let mut next_region = 0;

    for grid_row in grid {
        let mut current_row = Vec::new();
        let mut current_start = None;

        for (i, x) in grid_row.iter().enumerate() {
            if let Some(start) = current_start {
                // in a region
                if !x {
                    // this is the end of the region
                    let new_region = Region::new(start, i - start, 0);

                    // check the previous row to find if we intersect with a region
                    let intersecting_previous_regions = match &previous_row {
                        &Some(ref p) => p.iter()
                            .filter(|r| r.intersects_with(&new_region))
                            .collect(),
                        &None => Vec::new(),
                    };
                    let this_region = if intersecting_previous_regions.len() > 0 {
                        // for now, we'll just use the first label. If there's more than one
                        // we will need to unify the labels
                        let this_label = intersecting_previous_regions[0].label;
                        if intersecting_previous_regions.len() > 1 {
                            let the_labels = intersecting_previous_regions
                                .iter()
                                .map(|r| r.label)
                                .collect();
                            unify_regions_to_label(&mut rows, &the_labels, this_label);
                        }
                        this_label
                    } else {
                        let r = next_region;
                        next_region += 1;
                        r
                    };
                    current_row.push(new_region.with_label(this_region));
                    current_start = None;
                }
            } else {
                // not in a region
                if *x {
                    // a region has begun here
                    current_start = Some(i);
                }
            }
        }

        previous_row = Some(current_row.clone());
        rows.push(current_row);
    }

    rows
}

#[cfg(test)]
mod tests {
    mod get_regions {
        use super::super::*;

        #[test]
        fn row_with_no_regions() {
            let row = vec![false, false, false, false];
            let regions = get_regions_from_row(&row);
            assert_eq!(
                regions.len(),
                0,
                "No regions should be found in an empty row",
            );
        }

        #[test]
        fn row_which_is_all_region() {
            let row = vec![true, true, true, true];
            let regions = get_regions_from_row(&row);
            assert_eq!(
                regions,
                vec![Region::new(0, 4, 0)],
                "One region taking up the whole row",
            );
        }

        #[test]
        fn row_with_two_regions() {
            let row = vec![true, false, true, true];
            let regions = get_regions_from_row(&row);
            assert_eq!(regions, vec![Region::new(0, 1, 0), Region::new(2, 2, 1)]);
        }
    }

    mod connect_regions {
        use super::super::*;

        #[test]
        fn no_connections() {
            let regions = vec![vec![Region::new(0, 1, 0)], vec![Region::new(2, 1, 1)]];
            let connections = find_connected_regions(&regions);
            let mut group1 = HashSet::new();
            group1.insert(0);
            let mut group2 = HashSet::new();
            group2.insert(1);
            assert_eq!(connections, vec![group1, group2]);
        }

        #[test]
        fn one_connection() {
            let regions = vec![vec![Region::new(0, 2, 0)], vec![Region::new(1, 2, 1)]];
            let connections = find_connected_regions(&regions);
            let mut group1 = HashSet::new();
            group1.insert(0);
            group1.insert(1);
            assert_eq!(connections, vec![group1]);
        }

        #[test]
        fn connect_to_two_previous() {
            let regions = vec![
                vec![Region::new(0, 2, 0), Region::new(3, 2, 1)],
                vec![Region::new(1, 5, 2)],
            ];
            let connections = find_connected_regions(&regions);
            let mut group1 = HashSet::new();
            group1.insert(0);
            group1.insert(1);
            group1.insert(2);
            assert_eq!(connections, vec![group1]);
        }

        #[test]
        fn connect_to_two_previous_but_not_third() {
            let regions = vec![
                vec![
                    Region::new(0, 2, 0),
                    Region::new(3, 2, 1),
                    Region::new(7, 1, 3),
                ],
                vec![Region::new(1, 5, 2)],
            ];
            let connections = find_connected_regions(&regions);
            let mut group1 = HashSet::new();
            group1.insert(0);
            group1.insert(1);
            group1.insert(2);

            let mut group2 = HashSet::new();
            group2.insert(3);

            assert_eq!(connections, vec![group1, group2]);
        }

        #[test]
        fn connect_over_three_rows() {
            let regions = vec![
                vec![Region::new(0, 2, 0)],
                vec![Region::new(1, 2, 1)],
                vec![Region::new(0, 2, 2)],
            ];

            let connections = find_connected_regions(&regions);
            let mut group1 = HashSet::new();
            group1.insert(0);
            group1.insert(1);
            group1.insert(2);

            assert_eq!(connections, vec![group1]);
        }
    }
}
