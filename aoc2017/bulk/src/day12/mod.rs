use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use util::timed;

pub fn go() {
    let input = include_str!("input.txt");

    let map = parse_input(input);

    let (nodes, time) = timed(|| nodes_reachable_from(0, &map));

    println!("[{}ms] {} nodes are reachable from 0", time, nodes.len());

    let (groups, time) = timed(|| how_many_groups(&map));

    println!("[{}ms] {} groups exist", time, groups);
}

fn how_many_groups(map: &HashMap<u32, HashSet<u32>>) -> usize {
    let mut to_check = map.keys().cloned().collect::<HashSet<_>>();
    let mut result = 0;

    while to_check.len() > 0 {
        result += 1;
        let current_node = to_check.iter().next().unwrap().clone();
        to_check.remove(&current_node);
        for reachable in nodes_reachable_from(current_node, map) {
            to_check.remove(&reachable);
        }
    }

    result
}

fn nodes_reachable_from(node: u32, map: &HashMap<u32, HashSet<u32>>) -> HashSet<u32> {
    let mut to_visit = HashSet::new();
    let mut visited = HashSet::new();

    visited.insert(node);

    for n in map.get(&node).unwrap() {
        to_visit.insert(n);
    }

    while to_visit.len() > 0 {
        let n = to_visit.iter().next().unwrap().clone();
        match map.get(n) {
            Some(connections) => {
                visited.insert(*n);
                to_visit.remove(n);
                for c in connections.iter().filter(|c| !visited.contains(c)) {
                    to_visit.insert(c);
                }
            }
            None => {}
        }
    }

    visited
}

fn parse_input(input: &str) -> HashMap<u32, HashSet<u32>> {
    let lines = input.lines().filter_map(parse_input_line);

    let mut map = HashMap::new();

    for (node, links) in lines {
        map.insert(node, links);
    }

    map
}

fn parse_input_line(line: &str) -> Option<(u32, HashSet<u32>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) <-> (.+)").unwrap();
    }

    let c = RE.captures(line);
    match c {
        Some(cap) => {
            let num = u32::from_str(&cap[1]);
            match num {
                Ok(n) => Some((
                    n,
                    (&cap[2])
                        .trim()
                        .split(", ")
                        .filter_map(|n| u32::from_str(n).ok())
                        .collect::<HashSet<_>>(),
                )),
                Err(_) => None,
            }
        }
        None => None,
    }
}
