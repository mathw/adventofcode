use std::{collections::HashMap, fmt};

pub struct Ring<T> {
    nodes: HashMap<usize, RingNode<T>>,
}

#[derive(Debug)]
struct RingNode<T> {
    value: T,
    previous: usize,
    next: usize,
}

impl<T> Ring<T> {
    pub fn new<I: IntoIterator<Item = T>>(elems_iterator: I) -> Ring<T> {
        let mut nodes = HashMap::new();
        let mut previous_index = 0;
        for (index, elem) in elems_iterator.into_iter().enumerate() {
            nodes.insert(
                index,
                RingNode {
                    value: elem,
                    previous: previous_index,
                    next: index + 1,
                },
            );
            previous_index = index;
        }
        // and link the ends together...
        nodes.entry(0).and_modify(|n| n.previous = previous_index);
        nodes.entry(previous_index).and_modify(|n| n.next = 0);
        Ring { nodes }
    }

    pub fn remove_three_after(&mut self, node: &T) -> Option<(T, T, T)>
    where
        T: Eq,
    {
        if self.nodes.len() < 4 {
            return None;
        }
        let reference_node_next = self
            .nodes
            .values()
            .filter(|n| &n.value == node)
            .next()?
            .next;
        let first = self.nodes.remove(&reference_node_next)?;
        let second = self.nodes.remove(&first.next)?;
        let third = self.nodes.remove(&second.next)?;
        let mut reference_node = self.nodes.get_mut(&first.previous)?;
        reference_node.next = third.next;
        let mut fourth_node = self.nodes.get_mut(&third.next)?;
        fourth_node.previous = first.previous;

        Some((first.value, second.value, third.value))
    }

    pub fn insert_three_after(&mut self, reference: &T, three: (T, T, T)) -> Option<()>
    where
        T: Eq,
    {
        if self.nodes.len() < 1 {
            return None;
        }

        let (reference_node_index, reference_node_next) = self
            .nodes
            .iter()
            .filter(|(_, n)| &n.value == reference)
            .map(|(i, n)| (*i, n.next))
            .next()?;

        let indicies = self.find_three_insert_indicies();

        let first_new_node;
        let second_new_node;
        let third_new_node;

        {
            let mut next_node = self.nodes.get_mut(&reference_node_next)?;

            first_new_node = RingNode {
                value: three.0,
                previous: next_node.previous.clone(),
                next: indicies.1,
            };
            second_new_node = RingNode {
                value: three.1,
                previous: indicies.0,
                next: indicies.2,
            };
            third_new_node = RingNode {
                value: three.2,
                previous: indicies.1,
                next: reference_node_next,
            };
            next_node.previous = indicies.2;
        }

        self.nodes
            .entry(reference_node_index)
            .and_modify(|n| n.next = indicies.0);

        self.nodes.insert(indicies.0, first_new_node);
        self.nodes.insert(indicies.1, second_new_node);
        self.nodes.insert(indicies.2, third_new_node);

        Some(())
    }

    fn find_three_insert_indicies(&self) -> (usize, usize, usize) {
        let indicies = self.sorted_indices();
        let mut first_gap = 0;
        while indicies.contains(&first_gap) {
            first_gap += 1;
        }
        let mut second_gap = first_gap + 1;
        while indicies.contains(&second_gap) {
            second_gap += 1;
        }
        let mut third_gap = second_gap + 1;
        while indicies.contains(&third_gap) {
            third_gap += 1;
        }
        (first_gap, second_gap, third_gap)
    }

    fn sorted_indices(&self) -> Vec<usize> {
        let mut indicies = self.nodes.keys().cloned().collect::<Vec<usize>>();
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
        self.nodes.values().any(|n| n.value == *value)
    }

    pub fn highest_value(&self) -> Option<&T>
    where
        T: PartialOrd,
    {
        let mut highest = &self.nodes.values().next()?.value;
        for value in self.nodes.values() {
            if value.value > *highest {
                highest = &value.value;
            }
        }
        Some(highest)
    }

    pub fn value_after(&self, reference: &T) -> Option<&T>
    where
        T: Eq,
    {
        let reference_node = self
            .nodes
            .values()
            .filter(|n| n.value == *reference)
            .next()?;
        self.nodes.get(&reference_node.next).map(|n| &n.value)
    }
}

pub struct RingIter<'ring, T> {
    ring: &'ring Ring<T>,
    starting_index: usize,
    current_index: usize,
    has_iterated: bool,
}

impl<'ring, T> Iterator for RingIter<'ring, T> {
    type Item = &'ring T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_iterated && self.current_index == self.starting_index {
            return None;
        }
        self.has_iterated = true;
        let node = &self.ring.nodes.get(&self.current_index)?;
        self.current_index = node.next;
        return Some(&node.value);
    }
}

impl<T> fmt::Debug for Ring<T>
where
    T: fmt::Debug,
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
    assert_eq!(three, Some((2, 3, 4)));
    assert!(
        ring.insert_three_after(&5, three.unwrap()).is_some(),
        "insert should succeed"
    );
    println!("{:?}", ring.nodes);
    assert_eq!(
        format!("{:?}", ring),
        "[1, 5, 2, 3, 4, 6, ]",
        "after inserting"
    );
    assert!(
        ring.insert_three_after(&6, (7, 8, 9)).is_some(),
        "second insert should succeed"
    );
    assert_eq!(
        format!("{:?}", ring),
        "[1, 5, 2, 3, 4, 6, 7, 8, 9, ]",
        "after second inserting"
    );
}
