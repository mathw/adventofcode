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

    fn part2(&mut self, sender: &Sender<String>) {
        match determine_time(&self.dependencies, 60, 5) {
            Some(time) => sender.send(format!("The time taken is {}", time)).unwrap(),
            None => sender.send("Unable to determine time".into()).unwrap(),
        }
    }
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
    #[cfg(test)]
    println!("Starting node is {}", starting_node);

    let terminal_node = all_nodes.difference(&all_prerequisites).cloned().next()?;
    #[cfg(test)]
    println!("Terminal node is {}", terminal_node);

    let mut result = String::new();
    let mut completed: HashSet<char> = HashSet::new();
    completed.insert(starting_node);

    let mut available: HashSet<char> = starting_nodes.drain(0..).collect();
    available.remove(&starting_node);

    let mut current_node = starting_node;
    while current_node != terminal_node {
        let candidates = map_prerequisite_to_target.get(&current_node)?;
        #[cfg(test)]
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

        #[cfg(test)]
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
                #[cfg(test)]
                println!("No next move, ending loop");
                break;
            }
        }
    }

    result.push(current_node);

    Some(result)
}

fn build_prerequisite_maps(
    deps: &Vec<(char, char)>,
    map_prerequisite_to_target: &mut HashMap<char, Vec<char>>,
    map_target_to_prerequisite: &mut HashMap<char, Vec<char>>,
    all_nodes: &mut HashSet<char>,
    all_with_prerequisites: &mut HashSet<char>,
    all_prerequisites: &mut HashSet<char>,
) -> Option<char> {
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

    let terminal_node = all_nodes.difference(&all_prerequisites).cloned().next();
    #[cfg(test)]
    println!("Terminal node is {}", terminal_node.unwrap_or('0'));

    return terminal_node;
}

fn get_starting_nodes(
    all_nodes: &HashSet<char>,
    all_with_prerequisites: &HashSet<char>,
) -> Vec<char> {
    let mut starting_nodes: Vec<char> = all_nodes
        .difference(&all_with_prerequisites)
        .cloned()
        .collect();
    starting_nodes.sort();

    starting_nodes
}

fn determine_time(deps: &Vec<(char, char)>, step_time_factor: u32, workers: usize) -> Option<u32> {
    let mut map_prerequisite_to_target: HashMap<char, Vec<char>> = HashMap::new();
    let mut map_target_to_prerequisite: HashMap<char, Vec<char>> = HashMap::new();
    let mut all_nodes: HashSet<char> = HashSet::new();
    let mut all_prerequisites: HashSet<char> = HashSet::new();
    let mut all_with_prerequisites: HashSet<char> = HashSet::new();

    let terminal_node = build_prerequisite_maps(
        deps,
        &mut map_prerequisite_to_target,
        &mut map_target_to_prerequisite,
        &mut all_nodes,
        &mut all_with_prerequisites,
        &mut all_prerequisites,
    )?;

    let starting_nodes = get_starting_nodes(&all_nodes, &all_with_prerequisites);

    let mut completed: HashSet<char> = HashSet::new();
    completed.extend(starting_nodes.iter().cloned());

    let time_for_step = |c| time_for_letter(c) + step_time_factor;

    let mut time_taken = 0;

    let mut available: HashSet<char> = starting_nodes.iter().skip(workers).cloned().collect();

    let mut tasks: Vec<Task> = starting_nodes
        .into_iter()
        .take(workers)
        .map(|c| Task::new(c, time_for_step(c)))
        .collect();
    #[cfg(test)]
    println!("Starting tasks are {:?}", tasks);

    while !completed.contains(&terminal_node) {
        time_taken += 1;
        #[cfg(test)]
        println!("Time {}", time_taken);
        step(
            &mut tasks,
            &mut completed,
            &mut map_prerequisite_to_target,
            &mut available,
            &mut map_target_to_prerequisite,
            workers,
            time_for_step,
        );
    }

    Some(time_taken)
}

fn remove_newly_completed_tasks<'a>(
    newly_completed: impl Iterator<Item = &'a Task>,
    tasks: &mut Vec<Task>,
) {
    for c in newly_completed {
        if let Some(idx) = tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| *t == c)
            .map(|(i, _)| i)
            .next()
        {
            tasks.remove(idx);
        }
    }
}

fn get_candidates(
    newly_completed: &HashSet<Task>,
    map_prerequisite_to_target: &HashMap<char, Vec<char>>,
) -> Vec<char> {
    newly_completed
        .iter()
        .filter_map(|t| map_prerequisite_to_target.get(&t.name))
        .flat_map(|x| x)
        .cloned()
        .collect()
}

fn available_candidates<'a>(
    candidates: &'a std::vec::Vec<char>,
    map_target_to_prerequisite: &'a HashMap<char, Vec<char>>,
    completed: &'a HashSet<char>,
) -> impl Iterator<Item = &'a char> {
    candidates.iter().filter_map(move |c| {
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
    })
}

fn step<F>(
    tasks: &mut Vec<Task>,
    completed: &mut HashSet<char>,
    map_prerequisite_to_target: &mut HashMap<char, Vec<char>>,
    available: &mut HashSet<char>,
    map_target_to_prerequisite: &mut HashMap<char, Vec<char>>,
    workers: usize,
    time_for_step: F,
) where
    F: Fn(char) -> u32,
{
    let mut newly_completed = HashSet::<Task>::new();
    for t in tasks.iter_mut() {
        if t.tick() {
            newly_completed.insert(t.clone());
        }
    }

    if newly_completed.is_empty() {
        return;
    }

    #[cfg(test)]
    println!("Completed: {:?}", newly_completed);

    remove_newly_completed_tasks(newly_completed.iter(), tasks);

    completed.extend(newly_completed.iter().map(|t| t.name));

    let candidates: Vec<char> = get_candidates(&newly_completed, map_prerequisite_to_target);

    #[cfg(test)]
    println!(
        "Candidates {:?}, completed {:?}, available: {:?}, free workers: {}",
        candidates,
        completed,
        available,
        workers - tasks.len()
    );

    available.extend(available_candidates(
        &candidates,
        map_target_to_prerequisite,
        completed,
    ));

    let mut possible_moves: Vec<char> = available.iter().cloned().collect();
    possible_moves.sort();

    let next_moves: Vec<char> = possible_moves
        .iter()
        .take(workers - tasks.len())
        .cloned()
        .collect();

    for m in next_moves.iter() {
        available.remove(m);
    }

    #[cfg(test)]
    println!(
        "Possible moves: {:?}, next moves: {:?}",
        possible_moves, next_moves
    );

    if next_moves.is_empty() {
        return;
    }

    tasks.extend(next_moves.iter().map(|c| Task::new(*c, time_for_step(*c))));
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Task {
    name: char,
    time_remaining: u32,
}

impl Task {
    fn new(name: char, time: u32) -> Task {
        Task {
            name,
            time_remaining: time,
        }
    }

    fn tick(&mut self) -> bool {
        if self.time_remaining == 1 {
            self.time_remaining = 0;
            return true;
        } else {
            self.time_remaining -= 1;
            return false;
        }
    }
}

fn time_for_letter(c: char) -> u32 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'J' => 10,
        'K' => 11,
        'L' => 12,
        'M' => 13,
        'N' => 14,
        'O' => 15,
        'P' => 16,
        'Q' => 17,
        'R' => 18,
        'S' => 19,
        'T' => 20,
        'U' => 21,
        'V' => 22,
        'W' => 23,
        'X' => 24,
        'Y' => 25,
        'Z' => 26,
        _ => 0,
    }
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

    #[test]
    fn part_two_example() {
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

        let time = determine_time(&deps, 0, 2);

        assert_eq!(time, Some(15));
    }
}
