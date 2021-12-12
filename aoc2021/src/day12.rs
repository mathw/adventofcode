use crate::day::{DayResult, PartResult};
#[cfg(test)]
use maplit::hashset;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::hash::{Hash, Hasher};

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let caves = parse_caves(include_str!("inputs/day12.txt"))?;
    let part1 = caves.find_all_paths(&part1_okay_to_visit).len();
    let part2 = caves.find_all_paths(&part2_okay_to_visit).len();
    Ok(DayResult::new(
        PartResult::Success(format!("There are {} paths through the caves", part1)),
        PartResult::Success(format!(
            "There are {} paths through the caves by part 2 rules",
            part2
        )),
    ))
}

fn parse_edge(line: &str) -> Result<(&str, &str), String> {
    let parts = line.trim().split('-').collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(format!(
            "Input line \"{}\" has not got two parts after splitting by -",
            line
        ));
    }
    Ok((parts[0], parts[1]))
}

fn parse_caves(input: &str) -> Result<CaveSystem<'_>, String> {
    let edges = input
        .lines()
        .map(|l| parse_edge(l))
        .collect::<Result<Vec<_>, _>>()?;
    let mut caves = HashSet::new();
    let mut tunnels = HashSet::new();

    for (a, b) in edges {
        tunnels.insert(Tunnel::new(a, b));
        let cavea = Cave::new(a);
        let caveb = Cave::new(b);
        caves.insert(cavea);
        caves.insert(caveb);
    }

    Ok(CaveSystem { caves, tunnels })
}

fn is_big_label(l: &str) -> bool {
    l.chars().all(|c| c.is_uppercase())
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Cave<'a> {
    label: &'a str,
    is_big: bool,
}

impl<'a> Cave<'a> {
    fn new(label: &'a str) -> Self {
        Self {
            label,
            is_big: is_big_label(label),
        }
    }

    fn is_end_cave(&self) -> bool {
        self.label == "end"
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Tunnel<'a> {
    label1: &'a str,
    label2: &'a str,
}

impl<'a> Tunnel<'a> {
    fn new(label1: &'a str, label2: &'a str) -> Self {
        Self { label1, label2 }
    }

    fn connects_to(&self, destination: &str) -> bool {
        self.label1 == destination || self.label2 == destination
    }

    fn other_end_to(&self, label: &str) -> Option<&'a str> {
        if self.label1 == label {
            Some(self.label2)
        } else {
            if self.label2 == label {
                Some(self.label1)
            } else {
                None
            }
        }
    }
}

struct CaveSystem<'a> {
    caves: HashSet<Cave<'a>>,
    tunnels: HashSet<Tunnel<'a>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Path<'a> {
    path: VecDeque<Cave<'a>>,
    small_visits: HashMap<&'a str, usize>,
}

impl<'a> Hash for Path<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

impl<'a> Path<'a> {
    fn new() -> Self {
        Self {
            path: VecDeque::new(),
            small_visits: HashMap::new(),
        }
    }
    fn has_double_small_cave_visit(&self) -> bool {
        self.small_visits
            .values()
            .filter(|v| **v > 1)
            .next()
            .is_some()
    }
    fn num_times_visited(&self, label: &str) -> usize {
        *self.small_visits.get(label).unwrap_or(&0)
    }
    fn push(&mut self, cave: Cave<'a>) {
        if !cave.is_big {
            *self.small_visits.entry(cave.label).or_default() += 1;
        }
        self.path.push_back(cave);
    }
    fn back(&self) -> Option<&Cave<'a>> {
        self.path.back()
    }
    fn is_concluded(&self) -> bool {
        self.path.back().map(|c| c.is_end_cave()).unwrap_or(false)
    }
}

#[cfg(test)]
impl<'a> From<Vec<Cave<'a>>> for Path<'a> {
    fn from(v: Vec<Cave<'a>>) -> Self {
        let mut path = Path::new();
        for cave in v {
            path.push(cave);
        }
        path
    }
}

fn part1_okay_to_visit(path: &Path<'_>, label: &str) -> bool {
    if is_big_label(label) {
        true
    } else {
        path.num_times_visited(label) < 1
    }
}

fn part2_okay_to_visit(path: &Path<'_>, label: &str) -> bool {
    #[cfg(test)]
    {
        println!("Can {} be visited?", label);
        print_path(path);
    }

    if is_big_label(label) {
        return true;
    }

    if label == "start" || label == "end" {
        // can only visit start or end once each
        path.num_times_visited(label) < 1
    } else {
        // can only visit a small cave again if there isn't already a small cave visited twice
        path.num_times_visited(label) < 1 || !path.has_double_small_cave_visit()
    }
}

#[cfg(test)]
fn print_path(path: &Path<'_>) {
    for cave in path.path.iter() {
        print!("{}-", cave.label);
    }
    println!("");
}

impl<'a> CaveSystem<'a> {
    fn find_all_paths(
        &self,
        is_valid_cave_for_path: &(impl Fn(&Path<'a>, &str) -> bool + Sync),
    ) -> HashSet<Path<'a>> {
        let mut paths = HashSet::new();
        {
            let start_cave = self.starting_cave().expect("There must be a starting cave");
            let mut starting_path = Path::new();
            starting_path.push(start_cave);
            paths.insert(starting_path);
        }

        loop {
            #[cfg(test)]
            println!("### Loop start. {} existing paths", paths.len());
            if paths.len() == 0 {
                panic!("Paths are empty when starting new loop!");
            }
            let new_paths = paths
                .par_iter()
                .flat_map(|path| {
                    #[cfg(test)]
                    {
                        print!("## Searching to extend path ");
                        print_path(path);
                    }
                    let next_nodes = self.next_path_nodes(&path, &is_valid_cave_for_path);

                    #[cfg(test)]
                    println!("## Found {} next nodes {:?}", next_nodes.len(), next_nodes);

                    let mut new_paths = HashSet::new();

                    if next_nodes.len() == 0 {
                        #[cfg(test)]
                        println!("## no extension to this path possible. Pushing and moving on.");
                        new_paths.insert(path.clone());
                    }

                    for n in next_nodes {
                        let mut new_path = path.clone();
                        new_path.push(n);

                        #[cfg(test)]
                        {
                            print!("## New path: ");
                            print_path(&new_path);
                        }

                        new_paths.insert(new_path);
                    }
                    new_paths
                })
                .collect();

            if new_paths == paths {
                #[cfg(test)]
                println!("## No change to paths. Search complete.");
                break;
            }
            paths = new_paths;
        }

        paths = paths.into_iter().filter(|p| p.is_concluded()).collect();

        #[cfg(test)]
        {
            println!("=== conclusion: paths found:");
            for p in paths.iter() {
                print_path(p);
            }
        }

        paths
    }

    fn next_path_nodes(
        &self,
        path: &Path<'a>,
        is_valid_cave_for_path: &impl Fn(&Path<'a>, &str) -> bool,
    ) -> HashSet<Cave<'a>> {
        let last_cave = path
            .back()
            .expect("Must be called on a path with at least one node");

        if last_cave.is_end_cave() {
            #[cfg(test)]
            println!("The path reached the end cave. Stop.");
            return HashSet::new();
        }

        #[cfg(test)]
        println!("Searching from node {}", last_cave.label);

        let all_forward_tunnels = self
            .all_tunnels_for_cave(last_cave)
            .into_iter()
            .map(|t| t.other_end_to(last_cave.label).expect("Found a tunnel which doesn't connect to its own source cave - tunnel finder must be broken"))
            .filter(|l| is_valid_cave_for_path(path, l)).map(|l| Cave::new(l)).collect::<HashSet<_>>();
        all_forward_tunnels
    }

    fn all_tunnels_for_cave(&self, cave: &Cave<'a>) -> HashSet<Tunnel<'a>> {
        self.tunnels
            .iter()
            .filter(|t| t.connects_to(cave.label))
            .cloned()
            .collect()
    }

    fn starting_cave(&self) -> Option<Cave<'a>> {
        self.cave_with_label("start")
    }

    fn cave_with_label(&self, label: &str) -> Option<Cave<'a>> {
        self.caves
            .iter()
            .filter(|c| c.label == label)
            .cloned()
            .next()
    }
}

#[test]
fn test_very_simple() {
    let input = "start-a
a-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let paths = system.find_all_paths(&part1_okay_to_visit);
    assert_eq!(paths.len(), 1, "there is only one path");
    let expected_path: Path = vec![Cave::new("start"), Cave::new("a"), Cave::new("end")].into();
    assert_eq!(paths.into_iter().next().unwrap(), expected_path);
}

#[test]
fn test_part1_first_sample() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let paths = system.find_all_paths(&part1_okay_to_visit);
    assert_eq!(paths.len(), 10);
}

#[test]
fn test_part1_second_sample() {
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let system = parse_caves(input).expect("A cave system was expected");
    let paths = system.find_all_paths(&part1_okay_to_visit);
    assert_eq!(paths.len(), 19);
}

#[test]
fn test_part1_third_sample() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    let system = parse_caves(input).expect("A cave system was expected");
    let paths = system.find_all_paths(&part1_okay_to_visit);
    assert_eq!(paths.len(), 226);
}

#[test]
fn test_find_starting_cave() {
    let input = "start-a
a-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let start = system.starting_cave();
    assert_eq!(start, Some(Cave::new("start")));
}

#[test]
fn test_find_next_caves_simple() {
    let input = "start-a
a-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let start = system
        .starting_cave()
        .expect("A starting cave was expected");
    let mut path = Path::new();
    path.push(start);
    let next = system.next_path_nodes(&path, &part1_okay_to_visit);
    assert_eq!(next, hashset![Cave::new("a")])
}

#[test]
fn test_find_next_caves_simple_largecave() {
    let input = "start-A
A-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let start = system
        .starting_cave()
        .expect("A starting cave was expected");
    let mut path = Path::new();
    path.push(start);
    let next = system.next_path_nodes(&path, &part1_okay_to_visit);
    assert_eq!(next, hashset![Cave::new("A")])
}

#[test]
fn test_find_next_caves_dontgobacktosmall() {
    let input = "start-a
a-b
b-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let start = system
        .starting_cave()
        .expect("A starting cave was expected");
    let a = system.cave_with_label("a").expect("Cave a was expected");
    let mut path = Path::new();
    path.push(start);
    path.push(a);
    let next = system.next_path_nodes(&path, &part1_okay_to_visit);
    // should only find b, as we won't go back to start as it's a small cave in the path
    assert_eq!(next, hashset![Cave::new("b")])
}

#[test]
fn test_find_next_caves_dogobacktolarge() {
    let input = "start-A
A-b
b-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let start = system
        .starting_cave()
        .expect("A starting cave was expected");
    let a = system.cave_with_label("A").expect("Cave a was expected");
    let b = system.cave_with_label("b").expect("Cave b was expected");
    let mut path = Path::new();
    path.push(start);
    path.push(a);
    path.push(b);
    let next = system.next_path_nodes(&path, &part1_okay_to_visit);
    // should find A and end as A can be returned to as it's large
    assert_eq!(next, hashset![Cave::new("end"), Cave::new("A")])
}

#[test]
fn test_part2_first_sample() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let system = parse_caves(input).expect("A cave system was expected");
    let paths = system.find_all_paths(&part2_okay_to_visit);
    assert_eq!(paths.len(), 36);
}
