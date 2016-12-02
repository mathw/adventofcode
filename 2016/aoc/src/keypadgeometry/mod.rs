use instructions::keypad::Move;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keypad {
    pub position: char,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { position: '5' }
    }

    pub fn apply_move(&self, the_move: &Move) -> Keypad {
        Keypad {
            position: match the_move {
                &Move::Up => up_from(self.position),
                &Move::Down => down_from(self.position),
                &Move::Left => left_from(self.position),
                &Move::Right => right_from(self.position),
            },
        }
    }

    pub fn apply_moves<'a, I>(&self, moves: I) -> Keypad
        where I: Iterator<Item = &'a Move>
    {
        moves.fold((*self).clone(), |curr, m| curr.apply_move(m))
    }
}

fn up_from(pos: char) -> char {
    match pos {
        '1' => pos,
        '2' => pos,
        '3' => '1',
        '4' => pos,
        '5' => pos,
        '6' => '2',
        '7' => '3',
        '8' => '4',
        '9' => pos,
        'A' => '6',
        'B' => '7',
        'C' => '8',
        'D' => 'B',
        _ => pos,
    }
}

fn down_from(pos: char) -> char {
    match pos {
        '1' => '3',
        '2' => '6',
        '3' => '7',
        '4' => '8',
        '5' => pos,
        '6' => 'A',
        '7' => 'B',
        '8' => 'C',
        '9' => pos,
        'A' => pos,
        'B' => 'D',
        'C' => pos,
        'D' => pos,
        _ => pos,
    }
}

fn left_from(pos: char) -> char {
    match pos {
        '1' => pos,
        '2' => pos,
        '3' => '2',
        '4' => '3',
        '5' => pos,
        '6' => '5',
        '7' => '6',
        '8' => '7',
        '9' => '8',
        'A' => pos,
        'B' => 'A',
        'C' => 'B',
        'D' => pos,
        _ => pos,
    }
}

fn right_from(pos: char) -> char {
    match pos {
        '1' => pos,
        '2' => '3',
        '3' => '4',
        '4' => pos,
        '5' => '6',
        '6' => '7',
        '7' => '8',
        '8' => '9',
        '9' => pos,
        'A' => 'B',
        'B' => 'C',
        'C' => pos,
        'D' => pos,
        _ => pos,
    }
}
