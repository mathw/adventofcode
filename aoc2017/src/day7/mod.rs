mod parser;
mod fact;
mod tree;

use self::fact::Fact;
use self::tree::Node;
use std::collections::HashSet;
use std::collections::HashMap;
use util::timed_repeatedly;

pub fn go(reps: usize) {
    let input = parse_input(include_str!("input.txt"));

    let (result, time) = timed_repeatedly(reps, || part1(&input));
    println!("[{}ms] root name is {}", time, result);

    let (result, time) = timed_repeatedly(reps, || part2(&input, &result));
    println!("[{}ms] root name is {}", time, "foo");
}

fn part1(facts: &Vec<Fact>) -> String {
    find_root_name(&facts).unwrap()
}

fn part2(facts: &Vec<Fact>, root_name: &str) {
    let mut nodes = HashMap::new();

    // leaf nodes
    for node in facts.iter().filter(|fact| fact.underneath.len() == 0).map(|fact| Node::new(fact.name.clone(), fact.weight, Vec::new())) {
        nodes.insert(node.name.clone(), node);
    }

    // other nodes
    while !nodes.contains_key(root_name) {
        let keys = nodes.keys().cloned().collect::<HashSet<_>>();
        for fact in facts.iter().filter(|fact| fact.underneath.is_subset(&keys)) {
            let child_nodes = fact.underneath.iter().map(|name| nodes.remove(name).unwrap()).collect();
            nodes.insert(fact.name.clone(), Node::new(fact.name.clone(), fact.weight, child_nodes));
        }
    }

    // there's only one node in the map now, hopefully
    let root = nodes.remove(root_name).expect("No root!");

    for w in root.supporting.iter().map(|x| x.total_weight) {
        println!("{}", w);
    }
    // TODO find the single weight that's different, follow it to the end.

    let mut current_node = &root;
    while let Some(child) = current_node.get_unbalanced_child() {
        current_node = child;
    }


// TODO get the program to figure this out instead of requiring the user to do it by eye, but it's 00:25 and I need to go to bed
    println!("Child");
    for w in current_node.supporting.iter() {
        println!("{} total {}", w.self_weight, w.total_weight);
    }
}

fn parse_input(input: &str) -> Vec<Fact> {
    input.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Fact> {
    parser::parse(line)
}

fn find_root_name(facts: &[Fact]) -> Option<String> {
    let mut all_supported_names = HashSet::new();

    for name in facts.iter()
    {
        for n in name.underneath.iter() {
            all_supported_names.insert(n.clone());
        }
    }

    let all_names = facts.iter().map(|fact| fact.name.clone()).collect::<HashSet<_>>();

    let root_names = all_names.difference(&all_supported_names);

    root_names.cloned().next()
}