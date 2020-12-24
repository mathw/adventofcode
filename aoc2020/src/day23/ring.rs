use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

pub struct Ring<T>
where
    T: Eq + Hash + Clone,
{
    nodes: Vec<RingNode<T>>,
    lookup_nodes: HashMap<T, usize>,
    unused_indicies: HashSet<usize>,
}

#[derive(Debug, Clone)]
struct RingNode<T> {
    value: T,
    previous: usize,
    next: usize,
}

impl<T> Ring<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(elems: Vec<T>) -> Ring<T> {
        let mut nodes = Vec::with_capacity(elems.len());
        let mut lookup_nodes = HashMap::new();
        let mut previous_index = 0;
        for (index, elem) in elems.into_iter().enumerate() {
            nodes.insert(
                index,
                RingNode {
                    value: elem.clone(),
                    previous: previous_index,
                    next: index + 1,
                },
            );
            lookup_nodes.insert(elem, index);
            previous_index = index;
        }
        // and link the ends together...
        nodes.get_mut(0).unwrap().previous = previous_index;
        nodes.get_mut(previous_index).unwrap().next = 0;
        Ring {
            nodes,
            lookup_nodes,
            unused_indicies: HashSet::new(),
        }
    }

    fn index_of_value(&self, value: &T) -> Option<usize> {
        self.lookup_nodes.get(value).map(|i| *i)
    }

    pub fn remove_three_after(&mut self, node: &T) -> Option<(T, T, T)> {
        if self.nodes.len() < 4 {
            return None;
        }
        let reference_node_index = self.index_of_value(node)?;
        let reference_node_next = self.nodes[reference_node_index].next;
        let first = self.nodes[reference_node_next].clone();
        let second = self.nodes[first.next].clone();
        let third = self.nodes[second.next].clone();

        self.unused_indicies.insert(reference_node_next);
        self.unused_indicies.insert(first.next);
        self.unused_indicies.insert(second.next);

        self.nodes[reference_node_index].next = third.next;
        self.nodes[third.next].previous = first.previous;

        self.lookup_nodes.remove(&first.value);
        self.lookup_nodes.remove(&second.value);
        self.lookup_nodes.remove(&third.value);

        Some((first.value, second.value, third.value))
    }

    pub fn insert_three_after(&mut self, reference: &T, three: (T, T, T)) -> Option<()>
    where
        T: Eq,
    {
        if self.nodes.len() < 1 {
            return None;
        }

        let reference_node_index = self.index_of_value(reference)?;
        let reference_node_next = self.nodes[reference_node_index].next;

        let indicies = self.find_three_insert_indicies();

        let first_new_node;
        let second_new_node;
        let third_new_node;

        {
            let mut next_node = self.nodes.get_mut(reference_node_next)?;

            first_new_node = RingNode {
                value: three.0.clone(),
                previous: next_node.previous.clone(),
                next: indicies.1,
            };
            second_new_node = RingNode {
                value: three.1.clone(),
                previous: indicies.0,
                next: indicies.2,
            };
            third_new_node = RingNode {
                value: three.2.clone(),
                previous: indicies.1,
                next: reference_node_next,
            };
            next_node.previous = indicies.2;
        }

        self.nodes.get_mut(reference_node_index)?.next = indicies.0;

        self.nodes[indicies.0] = first_new_node;
        self.nodes[indicies.1] = second_new_node;
        self.nodes[indicies.2] = third_new_node;

        self.lookup_nodes.insert(three.0, indicies.0);
        self.lookup_nodes.insert(three.1, indicies.1);
        self.lookup_nodes.insert(three.2, indicies.2);

        self.unused_indicies.remove(&indicies.0);
        self.unused_indicies.remove(&indicies.1);
        self.unused_indicies.remove(&indicies.2);

        Some(())
    }

    fn find_three_insert_indicies(&self) -> (usize, usize, usize) {
        // this should always be the case in the crab game
        if self.unused_indicies.len() >= 3 {
            let mut i = self.unused_indicies.iter().take(3);
            let first = i.next().unwrap().clone();
            let second = i.next().unwrap().clone();
            let third = i.next().unwrap().clone();
            (first, second, third)
        } else {
            panic!("Not expecting to insert without removing three first")
        }
    }

    fn sorted_indices(&self) -> Vec<usize> {
        let mut indicies = (0..self.nodes.len())
            .into_iter()
            .filter(|i| !self.unused_indicies.contains(i))
            .collect::<Vec<usize>>();
        indicies.sort();
        indicies
    }

    pub fn iter(&self) -> RingIter<T> {
        let indicies = self.sorted_indices();
        return RingIter {
            ring: self,
            current_index: indicies[0],
            starting_index: indicies[0],
            has_iterated: false,
        };
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: Eq,
    {
        self.lookup_nodes.contains_key(value)
    }

    pub fn highest_value(&self) -> Option<&T>
    where
        T: PartialOrd,
    {
        let mut highest = self.lookup_nodes.keys().next()?;
        for value in self.lookup_nodes.keys() {
            if value > highest {
                highest = value;
            }
        }
        Some(highest)
    }

    pub fn value_after(&self, reference: &T) -> Option<&T>
    where
        T: Eq,
    {
        let reference_node_index = self.index_of_value(reference)?;
        let reference_node = self.nodes.get(reference_node_index)?;
        self.nodes.get(reference_node.next).map(|n| &n.value)
    }
}

#[derive(Debug)]
pub struct RingIter<'ring, T: Eq + Hash + Clone> {
    ring: &'ring Ring<T>,
    starting_index: usize,
    current_index: usize,
    has_iterated: bool,
}

impl<'ring, T> Iterator for RingIter<'ring, T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'ring T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_iterated && self.current_index == self.starting_index {
            return None;
        }
        self.has_iterated = true;
        let node = &self.ring.nodes.get(self.current_index)?;
        self.current_index = node.next;
        return Some(&node.value);
    }
}

impl<T> fmt::Debug for Ring<T>
where
    T: fmt::Debug + Eq + Hash + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[")?;
        for node in self.iter() {
            write!(f, "{:?}, ", node)?;
        }
        write!(f, "]")
    }
}

#[test]
fn test_ring() {
    let mut ring = Ring::new(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(format!("{:?}", ring), "[1, 2, 3, 4, 5, 6, ]");
    let three = ring.remove_three_after(&1);
    assert_eq!(format!("{:?}", ring), "[1, 5, 6, ]");
    assert_eq!(
        ring.unused_indicies,
        [1, 2, 3].iter().cloned().collect::<HashSet<usize>>()
    );
    assert_eq!(three, Some((2, 3, 4)));
    assert!(
        ring.insert_three_after(&5, three.unwrap()).is_some(),
        "insert should succeed"
    );
    assert_eq!(ring.unused_indicies.len(), 0);
    println!("{:?}", ring.nodes);
    assert_eq!(
        format!("{:?}", ring),
        "[1, 5, 2, 3, 4, 6, ]",
        "after inserting"
    );
}
