use instructions::keypad::Move;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keypad {
    pub position: u8,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { position: 5 }
    }

    pub fn new_with(num: u8) -> Option<Keypad> {
        if num > 0 && num < 10 {
            Some(Keypad { position: num })
        } else {
            None
        }
    }

    pub fn apply_move(&self, the_move: &Move) -> Keypad {
        Keypad {
            position: match the_move {
                &Move::Up => {
                    if is_top_row(self.position) {
                        self.position
                    } else {
                        self.position - 3
                    }
                }
                &Move::Down => {
                    if is_bottom_row(self.position) {
                        self.position
                    } else {
                        self.position + 3
                    }
                }
                &Move::Left => {
                    if is_left_side(self.position) {
                        self.position
                    } else {
                        self.position - 1
                    }
                }
                &Move::Right => {
                    if is_right_side(self.position) {
                        self.position
                    } else {
                        self.position + 1
                    }
                }
            },
        }
    }

    pub fn apply_moves<I>(&self, moves: I) -> Keypad
        where I: Iterator<Item = Move>
    {
        moves.fold((*self).clone(), |curr, m| curr.apply_move(&m))
    }
}

fn is_top_row(num: u8) -> bool {
    match num {
        1 => true,
        2 => true,
        3 => true,
        _ => false,
    }
}

fn is_bottom_row(num: u8) -> bool {
    match num {
        7 => true,
        8 => true,
        9 => true,
        _ => false,
    }
}

fn is_left_side(num: u8) -> bool {
    match num {
        1 => true,
        4 => true,
        7 => true,
        _ => false,
    }
}

fn is_right_side(num: u8) -> bool {
    match num {
        3 => true,
        6 => true,
        9 => true,
        _ => false,
    }
}


fn check_move_from(startpos: u8, the_move: &Move) -> u8 {
    Keypad::new_with(startpos).unwrap().apply_move(the_move).position
}

fn check_moves_from(startpos: u8, moves: Vec<Move>) -> u8 {
    Keypad::new_with(startpos).unwrap().apply_moves(moves.into_iter()).position
}

#[test]
fn test_moves_from_five() {
    assert_eq!(check_move_from(5, &Move::Up), 2);
    assert_eq!(check_move_from(5, &Move::Down), 8);
    assert_eq!(check_move_from(5, &Move::Left), 4);
    assert_eq!(check_move_from(5, &Move::Right), 6);
}

#[test]
fn test_moves_from_one() {
    assert_eq!(check_move_from(1, &Move::Up), 1);
    assert_eq!(check_move_from(1, &Move::Down), 4);
    assert_eq!(check_move_from(1, &Move::Left), 1);
    assert_eq!(check_move_from(1, &Move::Right), 2);
}

#[test]
fn test_moves_from_nine() {
    assert_eq!(check_move_from(9, &Move::Up), 6);
    assert_eq!(check_move_from(9, &Move::Down), 9);
    assert_eq!(check_move_from(9, &Move::Left), 8);
    assert_eq!(check_move_from(9, &Move::Right), 9);
}

#[test]
fn test_many_moves_from_five() {
    assert_eq!(check_moves_from(5,
                                vec![Move::Up,
                                     Move::Down,
                                     Move::Right,
                                     Move::Up,
                                     Move::Left,
                                     Move::Left,
                                     Move::Left]),
               1);
}
