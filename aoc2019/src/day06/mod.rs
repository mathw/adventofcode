use crate::day::Day;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{HashMap, HashSet};

pub struct Day6 {
    orbits: HashMap<&'static str, &'static str>,
}

impl Day6 {
    pub fn new() -> Result<Day6, String> {
        Ok(Day6 {
            orbits: parse_input(include_str!("input.txt")),
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

/// Takes input at one fact per line in format ORBITTED)ORBITER
/// Returns a map of orbiter -> orbitted
fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, &'a str> {
    input
        .lines()
        .map(|line| line.split(')'))
        .map(|mut pair| {
            let orbitted = pair.next().expect("orbitted body wasn't there");
            let orbiter = pair.next().expect("orbiting body wasn't there");
            (orbiter, orbitted)
        })
        .collect()
}

fn steps_to_common_centre<'a>(
    orbits: &HashMap<&'a str, &'a str>,
    object: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&cached) = cache.get(object) {
        return cached;
    }

    let mut steps = 0;

    if let Some(next) = orbits.get(object) {
        let rest = steps_to_common_centre(orbits, next, cache);
        steps += rest + 1;
    }

    cache.insert(object, steps);

    steps
}

/// Compute how many direct and indirect orbits are in the given orbit graph.
/// Visits each orbiter and computs the number of steps to the common centre
/// for each one, then adds them all together.
/// Passes a cache between calls to help avoid recalculating expensive things
fn total_orbits(orbits: &HashMap<&str, &str>) -> usize {
    let mut cache = HashMap::new();

    orbits
        .keys()
        .map(|o| steps_to_common_centre(orbits, o, &mut cache))
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
            // Unfortunately, this seems to be the only way to resolve the lifetimes
            // otherwise the `option` value can't be shown to live long enough
            // we have to collapse the iterator chain out of the option into a data structure
            // and then consume that
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
fn test_construct_orbits() {
    let map = parse_input(TEST_INPUT);
    assert_eq!(map.len(), 11);
    let b = map["B"];
    assert_eq!(b, "COM");

    let l = map["L"];
    assert_eq!(l, "K");
}

#[test]
fn test_common_centre() {
    let map = parse_input(TEST_INPUT);
    let mut cache = HashMap::new();
    let d = steps_to_common_centre(&map, "D", &mut cache);
    assert_eq!(d, 3, "D");
    let l = steps_to_common_centre(&map, "L", &mut cache);
    assert_eq!(l, 7, "L");
    let com = steps_to_common_centre(&map, "COM", &mut cache);
    assert_eq!(com, 0, "COM");
}

#[test]
fn test_total_orbits() {
    let map = parse_input(TEST_INPUT);
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
    let orbits = parse_input(TEST_INPUT_2);
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
