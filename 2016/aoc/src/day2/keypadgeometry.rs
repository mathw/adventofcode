use super::movet::Move;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeypadRules {
    up: HashMap<char, char>,
    down: HashMap<char, char>,
    left: HashMap<char, char>,
    right: HashMap<char, char>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keypad {
    pub position: char,
    rules: KeypadRules,
}

impl Keypad {
    pub fn new_square() -> Keypad {
        Keypad {
            position: '5',
            rules: normal_keypad_rules(),
        }
    }

    pub fn new_diamond() -> Keypad {
        Keypad {
            position: '5',
            rules: diamond_keypad_rules(),
        }
    }

    pub fn apply_move(&self, the_move: Move) -> Keypad {
        match the_move {
            Move::Up => self.do_move(&self.rules.up),
            Move::Down => self.do_move(&self.rules.down),
            Move::Left => self.do_move(&self.rules.left),
            Move::Right => self.do_move(&self.rules.right),
        }
    }

    pub fn apply_moves<'a, I>(&self, moves: I) -> Keypad
        where I: Iterator<Item = &'a Move>
    {
        moves.fold((*self).clone(), |curr, &m| curr.apply_move(m))
    }

    fn do_move(&self, moves: &HashMap<char, char>) -> Keypad {
        Keypad {
            position: move_by(moves, self.position),
            rules: self.rules.clone(),
        }
    }
}

fn move_by(moves: &HashMap<char, char>, position: char) -> char {
    *(moves.get(&position).unwrap_or(&position))
}

fn map_from_pairs<K, V, I: IntoIterator<Item = (K, V)>>(source: I) -> HashMap<K, V>
    where K: Hash + Eq,
          V: Hash
{
    let mut map = HashMap::new();

    for (key, value) in source {
        map.insert(key, value);
    }

    map
}

fn diamond_keypad_rules() -> KeypadRules {
    KeypadRules {
        up: map_from_pairs(vec![('3', '1'), ('6', '2'), ('7', '3'), ('8', '4'), ('A', '6'),
                                ('B', '7'), ('C', '8'), ('D', 'B')]),
        down: map_from_pairs(vec![('1', '3'), ('2', '6'), ('3', '7'), ('4', '8'), ('6', 'A'),
                                  ('7', 'B'), ('8', 'C'), ('B', 'D')]),
        left: map_from_pairs(vec![('3', '2'), ('4', '3'), ('6', '5'), ('7', '6'), ('8', '7'),
                                  ('9', '8'), ('B', 'A'), ('C', 'B')]),
        right: map_from_pairs(vec![('2', '3'), ('3', '4'), ('5', '6'), ('6', '7'), ('7', '8'),
                                   ('8', '9'), ('A', 'B'), ('B', 'C')]),
    }
}

fn normal_keypad_rules() -> KeypadRules {
    KeypadRules {
        up: map_from_pairs(vec![('4', '1'), ('5', '2'), ('6', '3'), ('7', '4'), ('8', '5'),
                                ('9', '6')]),
        down: map_from_pairs(vec![('1', '4'), ('2', '5'), ('3', '6'), ('4', '7'), ('5', '8'),
                                  ('6', '9')]),
        left: map_from_pairs(vec![('2', '1'), ('3', '2'), ('5', '4'), ('6', '5'), ('8', '7'),
                                  ('9', '8')]),
        right: map_from_pairs(vec![('1', '2'), ('2', '3'), ('4', '5'), ('5', '6'), ('7', '8'),
                                   ('8', '9')]),
    }
}
