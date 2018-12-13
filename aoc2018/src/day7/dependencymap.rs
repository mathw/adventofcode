use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct DependencyMap {
    map: HashMap<char, HashSet<char>>,
    completed: HashSet<char>,
}

impl DependencyMap {
    pub fn new(deps: impl IntoIterator<Item = (char, char)>) -> DependencyMap {
        let mut map = HashMap::new();
        for (required_for, required_by) in deps.into_iter() {
            let entry = map.entry(required_by).or_insert(HashSet::new());
            entry.insert(required_for);

            // make sure the dependency has an entry as well in case it's an origin node
            let _ = map.entry(required_for).or_insert(HashSet::new());
        }

        DependencyMap {
            map,
            completed: HashSet::new(),
        }
    }

    pub fn complete(&mut self, completed: char) {
        #[cfg(test)]
        println!("DependencyMap: marking {} completed", completed);

        self.map.remove(&completed);

        for (_, deps) in self.map.iter_mut() {
            deps.remove(&completed);
        }

        self.completed.insert(completed);
    }

    pub fn next_available(&self) -> Vec<char> {
        let mut result = Vec::new();

        for (key, deps) in self.map.iter() {
            if deps.is_empty() {
                result.push(key.clone());
            }
        }

        result.sort();

        #[cfg(test)]
        println!("DependencyMap: {:?} are available", result);

        result
    }

    pub fn is_finished(&self) -> bool {
        self.map.values().all(|v| v.is_empty())
    }
}

#[test]
fn empty_map_is_finished() {
    let v = Vec::new();
    let map = DependencyMap::new(v);

    assert_eq!(map.is_finished(), true);
}

#[test]
fn empty_map_has_none_available() {
    let v = Vec::new();
    let map = DependencyMap::new(v);

    assert_eq!(map.next_available(), Vec::<char>::new());
}

#[test]
fn simple_dep_chain() {
    let v = vec![('A', 'B'), ('B', 'C')];
    let mut map = DependencyMap::new(v);

    println!("{:?}", map);

    assert_eq!(
        map.next_available(),
        vec!('A'),
        "A should be first as it has no dependencies"
    );
    map.complete('A');
    assert_eq!(
        map.next_available(),
        vec!('B'),
        "B should be second once A is completed"
    );
    map.complete('B');
    assert_eq!(
        map.next_available(),
        vec!('C'),
        "C should be third once B is completed"
    );
    map.complete('C');
    assert_eq!(
        map.next_available(),
        Vec::<char>::new(),
        "Nothing is available after C is completed"
    );
}

#[test]
fn example_dep_chain() {
    let v = vec![
        ('C', 'A'),
        ('C', 'F'),
        ('A', 'B'),
        ('A', 'D'),
        ('B', 'E'),
        ('D', 'E'),
        ('F', 'E'),
    ];
    let mut map = DependencyMap::new(v);

    println!("{:?}", map);

    assert_eq!(
        map.next_available(),
        vec!('C'),
        "C should be available at the start"
    );
    map.complete('C');
    assert_eq!(
        map.next_available(),
        vec!('A', 'F'),
        "A and F should be available after C"
    );
    map.complete('A');
    assert_eq!(
        map.next_available(),
        vec!('B', 'D', 'F'),
        "B, D and F should be available after A and C"
    );
    map.complete('B');
    assert_eq!(
        map.next_available(),
        vec!('D', 'F'),
        "D and F should be available after A and B and C"
    );
    map.complete('D');
    assert_eq!(
        map.next_available(),
        vec!('F'),
        "F should be available after A and B and C and D"
    );
    map.complete('F');
    assert_eq!(
        map.next_available(),
        vec!('E'),
        "E should be available after everything else"
    );
    map.complete('E');
    assert_eq!(
        map.next_available(),
        Vec::<char>::new(),
        "Nothing should be left after E"
    );
}
