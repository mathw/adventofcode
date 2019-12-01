extern crate util;

use util::powerset::PowerSet;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::hash::Hash;

fn main() {
    let input = include_str!("input.txt");
    let components = parse_components(input);
    // let bridge = strongest_bridge_from_components(&components);
    // println!("Strongest bridge is {}", bridge.strength());
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Component(u32, u32);

impl Component {
    fn can_connect_to(&self, pin: u32) -> bool {
        self.0 == pin || self.1 == pin
    }
}

impl FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Component, Self::Err> {
        let parts = s.trim().split("/").collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(())
        } else {
            let a = u32::from_str(parts[0]).map_err(|_| ())?;
            let b = u32::from_str(parts[1]).map_err(|_| ())?;
            Ok(Component(a, b))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Bridge {
    components: LinkedList<Component>,
    start_pin: u32,
    end_pin: u32
}

impl Bridge {
    fn strength(&self) -> u32 {
        self.components.iter().map(|c| c.0 + c.1).sum()
    }

    fn new(component: Component) -> Bridge {
        Bridge {
            components: { let mut l = LinkedList::new(); l.push_back(component); l },
            start_pin: component.0,
            end_pin: component.1
        }
    }

    fn from_iter<I>(i: I) -> Option<Bridge>
    where I: IntoIterator<Item = Component> {
        let mut iter = i.into_iter();
        let first_item = iter.next();
        if first_item.is_none() {
            return None;
        }

        let mut bridge = Some(Bridge::new(first_item.unwrap()));
        for c in iter {
            bridge = bridge.and_then(|b| b.add_to_end(c));
            if bridge.is_none() {
                return None;
            }
        }

        bridge
    }

    fn start_pin(&self) -> u32 {
        self.start_pin
    }

    fn end_pin(&self) -> u32 {
        self.end_pin
    }

    fn add_to_start(&self, c: Component) -> Option<Bridge> {
        if c.can_connect_to(self.start_pin) {
            let mut r = self.clone();
            r.components.push_front(c);
            r.start_pin = if c.0 == self.start_pin { c.1 } else { c.0 };
            Some(r)
        }
        else {
            None
        }
    }

    fn add_to_end(&self, c: Component) -> Option<Bridge> {
        if c.can_connect_to(self.end_pin) {
            let mut r = self.clone();
            r.components.push_back(c);
            r.end_pin = if c.0 == self.end_pin { c.1 } else { c.0 };
            Some(r)
        }
        else {
            None
        }
    }
}

fn can_make_valid_bridge(components: &[Component]) -> bool {
    let mut compset = components
        .into_iter()
        .cloned()
        .collect::<HashSet<Component>>();
    let mut bridge = vec![];
    let mut is_currently_valid = false;

    loop {
        if compset.len() == 0 {
            // we're out of components!
            return is_currently_valid;
        }

        if bridge.len() == 0 {
            // we haven't considered any components yet
            let c = *compset.iter().next().unwrap();
            bridge.push(c);
            compset.remove(&c);

            if compset.len() == 0 {
                // a one-element bridge is obviously valid
                return true;
            }
        }

        // can we put one on the end?
        let last_component = bridge[bridge.len() - 1];
        let candidates = find_components_which_connect_to(last_component.1, &compset);
    }

    is_currently_valid
}

/// Add one component to the given bridge
/// Is non-deterministic, so returns a collection of new bridges with every possible new component added
/// accompanied by their leftover components.
fn extend_bridge_from_components(bridge: &Bridge, components: &HashSet<Component>) -> Vec<(Bridge, HashSet<Component>)> {
    let start_possibles = find_components_which_connect_to(bridge.start_pin, components);
    let end_possibles = find_components_which_connect_to(bridge.end_pin, components);

    let starts = start_possibles.iter().map(|&sp| {
        (bridge.add_to_start(sp).expect("Something went wrong when I can't add to the start of the bridge here"),
         components.difference(&set_of_one(sp)).cloned().collect::<HashSet<_>>())
    });
    let ends = end_possibles.iter().map(|&ep| {
        (bridge.add_to_end(ep).expect("Something went wrong when I can't add to the end of the bridge here"),
         components.difference(&set_of_one(ep)).cloned().collect::<HashSet<_>>())
    });

    starts.chain(ends).collect()
}

fn build_all_bridges(components: &HashSet<Component>) -> Vec<Bridge> {
    let mut working = Vec::new();

    for c in components {
        let remainder = set_without(components, *c);
        let bridge = Bridge::new(*c);
        working.extend(extend_bridge_from_components(&bridge, &remainder));
    }

    let mut result = Vec::new();

    while working.len() > 0 {
        let mut working2 = Vec::new();
        for (bridge, remainder) in working.drain(0..) {
            if remainder.len() == 0 {
                result.push(bridge);
            }
            else {
                working2.extend(extend_bridge_from_components(&bridge, &remainder));
            }
        }
        working.extend(working2.drain(0..));
    }

    result
}

fn set_without<T>(set: &HashSet<T>, x: T) -> HashSet<T> where T: Hash + Eq + Clone {
    set.difference(&set_of_one(x)).cloned().collect()
}

fn set_of_one<T>(x: T) -> HashSet<T>
where T: Hash + Eq {
    let mut s = HashSet::new();
    s.insert(x);
    s
}

fn find_components_which_connect_to(
    pin: u32,
    components: &HashSet<Component>,
) -> HashSet<Component> {
    components
        .iter()
        .filter(|c| c.0 == pin || c.1 == pin)
        .cloned()
        .collect()
}

fn parse_components(input: &str) -> HashSet<Component> {
    input
        .lines()
        .filter_map(|line| Component::from_str(line).ok())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_all() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        let components = parse_components(input);

        let bridges = build_all_bridges(&components);
        let bridges = bridges.iter().cloned().collect::<HashSet<Bridge>>();

        let expected_bridges = vec![
            Bridge::from_iter(vec![Component(0, 1)]),
            Bridge::from_iter(vec![Component(0, 1), Component(10, 1)]),
            Bridge::from_iter(vec![Component(0, 1), Component(10, 1), Component(9, 10)]),
            Bridge::from_iter(vec![Component(0, 2)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 3)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 3), Component(3, 4)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 3), Component(3, 5)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 2)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 2), Component(2, 3)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 2), Component(2, 3), Component(3, 4)]),
            Bridge::from_iter(vec![Component(0, 2), Component(2, 2), Component(2, 3), Component(3, 5)]),
        ].iter().map(|o| o.clone().unwrap()).collect::<HashSet<Bridge>>();

        assert_eq!(bridges.len(), expected_bridges.len(), "The right number of bridges must be produced");

        assert_eq!(bridges, expected_bridges);
    }
}