use crate::day::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

pub struct Day7 {
    dependencies: Vec<(char, char)>,
}

impl Day7 {
    pub fn new() -> Option<Day7> {
        let deps = include_str!("input.txt")
            .lines()
            .map(parse_dependency)
            .collect::<Option<Vec<(char, char)>>>()?;
        Some(Day7 { dependencies: deps })
    }
}

impl Day for Day7 {
    fn part1(&mut self, sender: &Sender<String>) {
        match determine_order(&self.dependencies) {
            Some(order) => sender.send(format!("The order is {}", order)).unwrap(),
            None => sender
                .send(format!("No order could be determined"))
                .unwrap(),
        }
    }

    fn part2(&mut self, sender: &Sender<String>) {}
}

fn parse_dependency(line: &str) -> Option<(char, char)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Step (.) must be finished before step (.) can begin\.")
            .expect("Regex should be parseable as I typed it myself");
    }

    let cap = RE.captures_iter(line).next()?;

    let dep = &cap[1].chars().next()?;
    let trg = &cap[2].chars().next()?;
    Some((*dep, *trg))
}

fn determine_order(deps: &Vec<(char, char)>) -> Option<String> {
    let mut map_prerequisite_to_target: HashMap<char, Vec<char>> = HashMap::new();
    let mut map_target_to_prerequisite: HashMap<char, Vec<char>> = HashMap::new();
    let mut all_nodes: HashSet<char> = HashSet::new();
    let mut all_prerequisites: HashSet<char> = HashSet::new();
    let mut all_with_prerequisites: HashSet<char> = HashSet::new();

    for (prerequisite, wanted) in deps.iter() {
        all_nodes.insert(*prerequisite);
        all_nodes.insert(*wanted);

        all_prerequisites.insert(*prerequisite);
        all_with_prerequisites.insert(*wanted);

        #[cfg(test)]
        println!("{} is a prerequisite for {}", prerequisite, wanted);
        let entry = map_prerequisite_to_target
            .entry(*prerequisite)
            .or_insert(Vec::new());
        entry.push(*wanted);

        let entry = map_target_to_prerequisite
            .entry(*wanted)
            .or_insert(Vec::new());
        entry.push(*prerequisite);
    }

    for (_, val) in map_prerequisite_to_target.iter_mut() {
        val.sort();
    }

    #[cfg(test)]
    println!(
        "Nodes with prerequisites: {:?}\nNodes which are prerequisites: {:?}",
        all_with_prerequisites, all_prerequisites
    );

    let mut starting_nodes: Vec<char> = all_nodes
        .difference(&all_with_prerequisites)
        .cloned()
        .collect();
    starting_nodes.sort();
    let starting_node = starting_nodes[0];
    // #[cfg(test)]
    println!("Starting node is {}", starting_node);

    let terminal_node = all_nodes.difference(&all_prerequisites).cloned().next()?;
    // #[cfg(test)]
    println!("Terminal node is {}", terminal_node);

    let mut result = String::new();
    let mut completed: HashSet<char> = HashSet::new();
    completed.insert(starting_node);

    let mut available: HashSet<char> = starting_nodes.drain(0..).collect();
    available.remove(&starting_node);

    let mut current_node = starting_node;
    while current_node != terminal_node {
        let candidates = map_prerequisite_to_target.get(&current_node)?;
        // #[cfg(test)]
        println!(
            "Current node is {} with candidates {:?}, completed {:?}, available: {:?}",
            current_node, candidates, completed, available
        );
        result.push(current_node);

        available.extend(candidates.iter().filter_map(|c| {
            let prerequisites: HashSet<char> = map_target_to_prerequisite
                .get(c)?
                .iter()
                .filter(|p| !completed.contains(*p))
                .cloned()
                .collect();
            if prerequisites.is_empty() {
                Some(c)
            } else {
                None
            }
        }));

        let mut possible_moves: Vec<char> = available.iter().cloned().collect();
        possible_moves.sort();

        let next_move = possible_moves.first();

        // #[cfg(test)]
        println!(
            "Possible moves: {:?}, next move: {:?}",
            possible_moves, next_move
        );

        match next_move {
            Some(n) => {
                current_node = *n;
                completed.insert(current_node);
                available.remove(&current_node);
            }
            None => {
                // #[cfg(test)]
                println!("No next move, ending loop");
                break;
            }
        }
    }

    result.push(current_node);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let deps = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .lines()
            .map(parse_dependency)
            .collect::<Option<Vec<(char, char)>>>()
            .expect("Example should parse");

        assert_eq!(
            deps.len(),
            7,
            "Example should have seven dependency statements"
        );

        assert_eq!(
            deps,
            vec![
                ('C', 'A'),
                ('C', 'F'),
                ('A', 'B'),
                ('A', 'D'),
                ('B', 'E'),
                ('D', 'E'),
                ('F', 'E')
            ],
            "Dependencies must have parsed correctly"
        );

        let order = determine_order(&deps);

        assert_eq!(order, Some("CABDFE".into()));
    }
}
