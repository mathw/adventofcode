use std::hash::Hash;
use util::knot_hash;
use util::timed;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn go() {
    let input = "hwlqcszp";

    let (result, time) = timed(|| count_used_in_grid(input));
    println!("[{}ms] {} squares used in the grid", time, result);

    let (regions, time) = timed(|| {
        let rows = (0..128).map(|r| row(input, r)).collect();
        let mut grid = grid_to_map(&rows);
        label_grid_regions(&mut grid);
        count_unique_regions(&grid)
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

/// Convert the grid of filled squares to a representation of coord -> labelled square
fn grid_to_map(grid: &Vec<Vec<bool>>) -> HashMap<(usize, usize), (bool, Option<u16>)> {
    let mut map = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            map.insert((x, y), (*spot, None));
        }
    }

    map
}

/// Go through the whole grid, kicking off flood-fill labelling of anything
/// we find which is occupied and not yet labelled
fn label_grid_regions(grid: &mut HashMap<(usize, usize), (bool, Option<u16>)>) {
    let mut next_region_number = 0;

    for x in 0..128 {
        for y in 0..128 {
            if let Some(&(true, None)) = grid.get(&(x, y)) {
                // an unlabelled, occupied square! Commence the labelling!
                label_fill(grid, x, y, next_region_number);
                next_region_number += 1;
            }
        }
    }
}

fn label_fill(
    grid: &mut HashMap<(usize, usize), (bool, Option<u16>)>,
    x: usize,
    y: usize,
    label: u16,
) {
    // slightly torturous two-function solution in order to avoid recursing into a stack overflow
    let mut to_process = label_fill_worker(grid, x, y, label);

    loop {
        // oh the mess we make waiting for NLL
        let process_queue_size = to_process.len().clone();
        if process_queue_size > 0 {
            let new_process = to_process
                .iter()
                .flat_map(|&(xx, yy)| label_fill_worker(grid, xx, yy, label))
                .collect();
            to_process = new_process;
        } else {
            break;
        }
    }
}

/// Helper for label_fill so it doesn't have to go fully recursive and overflow the stack
fn label_fill_worker(
    grid: &mut HashMap<(usize, usize), (bool, Option<u16>)>,
    x: usize,
    y: usize,
    label: u16,
) -> Vec<(usize, usize)> {
    {
        let se = grid.get_mut(&(x, y));
        if let Some(start_entry) = se {
            if !start_entry.0 {
                // this is not an occupied square - abort
                return Vec::new();
            }

            if start_entry.1 == Some(label) {
                // already labelled with this label - abort
                return Vec::new();
            }

            start_entry.1 = Some(label);
        }
    }

    neighbours_of(x, y)
}

/// Return the valid neighbour coordinates of a given x/y pair
fn neighbours_of(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut ns = Vec::new();

    if x > 0 {
        ns.push(((x - 1), y));
    }

    if y > 0 {
        ns.push((x, (y - 1)));
    }

    if x < 127 {
        ns.push(((x + 1), y));
    }

    if y < 127 {
        ns.push((x, (y + 1)));
    }

    ns
}

fn count_unique_regions<K>(grid: &HashMap<K, (bool, Option<u16>)>) -> usize
where
    K: Eq + Hash,
{
    grid.values()
        .filter_map(|&(_, l)| l)
        .collect::<HashSet<u16>>()
        .len()
}
