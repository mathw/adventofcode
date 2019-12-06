use crate::day::Day;
use pathfinding::directed::{dfs::dfs, dijkstra::dijkstra};
use std::collections::{HashMap, HashSet};

pub struct Day6 {
    orbits: HashMap<&'static str, &'static str>,
}

impl Day6 {
    pub fn new() -> Result<Day6, String> {
        Ok(Day6 {
            orbits: construct_orbits(parse_input(include_str!("input.txt"))),
        })
    }
}

impl Day for Day6 {
    fn part1(&mut self) -> Result<String, String> {
        Ok(format!("{} total orbits", total_orbits(&self.orbits)))
    }

    fn part2(&mut self) -> Result<String, String> {
        let (_path, transfers) =
            transfers_from_to(&self.orbits, "YOU", "SAN").ok_or("No path found".to_owned())?;
        Ok(format!("{} transfers from me to Santa", transfers))
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
    input.lines().map(|line| line.split(')')).map(|mut pair| {
        (
            pair.next().expect("first wasn't there"),
            pair.next().expect("second wasn't there"),
        )
    })
}

fn construct_orbits<'a>(
    data: impl Iterator<Item = (&'a str, &'a str)>,
) -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();

    for (orbitted, orbiter) in data {
        map.insert(orbiter, orbitted);
    }

    map
}

fn steps_to_common_centre<'a>(orbits: &HashMap<&'a str, &'a str>, object: &'a str) -> usize {
    let result = dfs(
        object,
        |o| orbits.get(o).into_iter().cloned(),
        |o| !orbits.contains_key(o),
    );
    match result {
        None => 0,
        Some(path) => path.len() - 1,
    }
}

fn total_orbits(orbits: &HashMap<&str, &str>) -> usize {
    orbits
        .keys()
        .map(|o| steps_to_common_centre(orbits, o))
        .sum()
}

/// Compute how many transfers are required to move from the object orbited by `from` to
/// the object orbited by `to`.
fn transfers_from_to<'a>(
    orbits: &HashMap<&'a str, &'a str>,
    from: &str,
    to: &str,
) -> Option<(Vec<&'a str>, usize)> {
    let from_orbits = orbits.get(from)?;
    let to_orbits = orbits.get(to)?;
    let transfer_map = make_transfer_map(&orbits);

    let r = dijkstra(
        from_orbits,
        |o| {
            let option = transfer_map.get(o);
            let option_iter = option.iter();
            let elements_iter = option_iter.map(|x| x.iter()).flatten().map(|x| (*x, 1));
            let v: Vec<_> = elements_iter.collect();
            v.into_iter()
        },
        |o| o == to_orbits,
    )?;

    Some(r.clone())
}

fn make_transfer_map<'a>(orbits: &HashMap<&'a str, &'a str>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut result = HashMap::new();
    for (orbiter, orbitted) in orbits {
        (*result.entry(*orbitted).or_insert(HashSet::new())).insert(*orbiter);
        (*result.entry(*orbiter).or_insert(HashSet::new())).insert(*orbitted);
    }

    result
}

#[cfg(test)]
const TEST_INPUT: &'static str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

#[test]
fn test_parse_input() {
    let result = parse_input(TEST_INPUT).collect::<Vec<_>>();
    assert_eq!(result.len(), 11, "11 inputs should be parsed");
    assert_eq!(result[0], ("COM", "B"));
    assert_eq!(result[5], ("B", "G"));
}

#[test]
fn test_construct_orbits() {
    let iter = parse_input(TEST_INPUT);
    let map = construct_orbits(iter);
    assert_eq!(map.len(), 11);
    let b = map["B"];
    assert_eq!(b, "COM");

    let l = map["L"];
    assert_eq!(l, "K");
}

#[test]
fn test_common_centre() {
    let iter = parse_input(TEST_INPUT);
    let map = construct_orbits(iter);
    let d = steps_to_common_centre(&map, "D");
    assert_eq!(d, 3, "D");
    let l = steps_to_common_centre(&map, "L");
    assert_eq!(l, 7, "L");
    let com = steps_to_common_centre(&map, "COM");
    assert_eq!(com, 0, "COM");
}

#[test]
fn test_total_orbits() {
    let iter = parse_input(TEST_INPUT);
    let map = construct_orbits(iter);
    let total = total_orbits(&map);
    assert_eq!(total, 42);
}

#[cfg(test)]
const TEST_INPUT_2: &'static str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

#[test]
fn test_transfers() {
    let orbits = construct_orbits(parse_input(TEST_INPUT_2));
    if let Some((_path, cost)) = transfers_from_to(&orbits, "YOU", "SAN") {
        assert_eq!(cost, 4);
    } else {
        assert!(false, "Didn't get a path");
    }
}

#[test]
fn test_make_transfer_map() {
    let mut orbits = HashMap::new();
    orbits.insert("a", "b");
    orbits.insert("c", "b");
    let map = make_transfer_map(&orbits);

    let from_b = &map["b"];
    assert!(from_b.contains("a"));
    assert!(from_b.contains("c"));
}
