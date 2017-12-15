use util::knot_hash;
use util::timed;
use std::collections::HashSet;

pub fn go() {
    let input = "hwlqcszp";

    let (result, time) = timed(|| count_used_in_grid(input));
    println!("[{}ms] {} squares used in the grid", time, result);

    let (regions, time) = timed(|| {
        let rows = (0..128).map(|r| row(input, r));
        let regions = rows.map(|r| get_regions_from_row(&r)).collect::<Vec<_>>();
        let connected_groups = find_connected_regions(&regions);
        connected_groups.len()
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

    fn with_label(&self, label: u16) -> Region {
        Region {
            label: label,
            start_index: self.start_index,
            length: self.length,
        }
    }

    fn end_index(&self) -> usize {
        self.start_index + self.length
    }

    fn intersects_with(&self, other: &Region) -> bool {
        (self.start_index <= other.start_index && self.end_index() > other.start_index)
            || (self.start_index < other.end_index() && self.end_index() > other.start_index)
    }
}

fn get_regions_from_row<I>(row: &I) -> Vec<Region>
where
    I: IntoIterator<Item = bool> + Clone,
{
    let mut current_start = None;
    let mut regions = Vec::new();
    let mut region_label = 0;

    for (i, x) in row.clone().into_iter().enumerate() {
        if let Some(start) = current_start {
            // in a region
            if !x {
                // this is the end of the region
                regions.push(Region::new(start, i - start, region_label));
                region_label += 1;
                current_start = None;
            }
        } else {
            // not in a region
            if x {
                // a region has begun here
                current_start = Some(i);
            }
        }
    }

    // at the end of the row we have to check if we're in a region we need to finish up
    if let Some(start) = current_start {
        let end = row.clone().into_iter().count() - start;
        regions.push(Region::new(start, end, region_label));
    }

    regions
}

fn find_connected_regions<I>(rows: &I) -> Vec<HashSet<u16>>
where
    I: IntoIterator<Item = Vec<Region>> + Clone,
{
    let mut result = Vec::<HashSet<u16>>::new();

    for (row_index, row) in rows.clone().into_iter().enumerate() {
        for region in row.iter() {
            let connected_here = find_connected_previous(region, row_index, rows.clone());
            for c in result.iter_mut() {
                if c.intersection(&connected_here).count() > 0 {
                    for x in connected_here.clone() {
                        c.insert(x);
                    }
                }
            }
            result.push(connected_here);
        }
    }

    let mut unique = Vec::new();
    for set in result.iter().filter(|x| x.len() > 0).cloned() {
        if !unique.iter().any(|x| *x == set) {
            unique.push(set);
        }
    }

    unique
}

fn find_connected_previous<I>(region: &Region, row_index: usize, rows: I) -> HashSet<u16>
where
    I: IntoIterator<Item = Vec<Region>>,
{
    let mut set = HashSet::new();
    set.insert(region.label);

    if row_index > 0 {
        let previous_row_index = row_index - 1;
        for r in rows.into_iter()
            .nth(previous_row_index)
            .unwrap()
            .iter()
            .filter(|r| r.intersects_with(region))
        {
            set.insert(r.label);
        }
    }

    set
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
