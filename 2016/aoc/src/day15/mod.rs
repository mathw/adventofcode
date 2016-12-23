pub fn do_day15() {
    let discs = discs_for_puzzle();
    let time = rotate_until_passable(&discs);

    println!("Drop at {}", time);

    let discs = discs_for_part_two();
    let time = rotate_until_passable(&discs);
    println!("(part two) Drop at {}", time);
}
/// A disc with a number of positions
#[derive(PartialEq, Eq, Clone, Debug)]
struct Disc {
    positions: u32,
    current: u32,
}

impl Disc {
    fn new(positions: u32, current: u32) -> Disc {
        Disc {
            positions: positions,
            current: current,
        }
    }

    fn rotate(&self, steps: u32) -> Disc {
        Disc { current: (self.current + steps) % self.positions, ..*self }
    }

    fn passable_in_steps(&self) -> u32 {
        if self.current == 0 {
            0
        } else {
            self.positions - self.current
        }
    }
}

fn discs_for_puzzle() -> Vec<Disc> {
    vec![Disc::new(13, 10),
         Disc::new(17, 15),
         Disc::new(19, 17),
         Disc::new(7, 1),
         Disc::new(5, 0),
         Disc::new(3, 1)]
}

fn discs_for_part_two() -> Vec<Disc> {
    let mut d = discs_for_puzzle();
    d.push(Disc::new(11, 0));
    d
}

fn discs_for_sample() -> Vec<Disc> {
    vec![Disc::new(5, 4), Disc::new(2, 1)]
}

/// Can a capsule dropped at time = 0 pass these discs?
fn can_pass_discs(discs: &Vec<Disc>) -> bool {
    fn can_pass(discs: &Vec<Disc>, idx: usize, want: u32) -> bool {
        let ref disc = discs[idx];
        // println!("This disc is at {} of {} passable in {} and I want it to be passable in {}",
        //          disc.current,
        //          disc.positions,
        //          disc.passable_in_steps(),
        //          want);
        if disc.passable_in_steps() == want % disc.positions {
            if idx == discs.len() - 1 {
                true
            } else {
                can_pass(discs, idx + 1, want + 1)
            }
        } else {
            false
        }
    }

    can_pass(discs, 0, 1)
}

fn rotate_all_discs(discs: &Vec<Disc>, steps: u32) -> Vec<Disc> {
    discs.iter().map(|d| d.rotate(steps)).collect()
}

/// Return the number of time iterations required to get the discs to be passable
fn rotate_until_passable(discs: &Vec<Disc>) -> u32 {
    let mut count = 0;
    let mut discs = discs.clone();
    loop {
        // println!("Now considering time {}", count);
        if can_pass_discs(&discs) {
            return count;
        }

        // println!("I need to rotate");
        let steps = discs[0].passable_in_steps();
        // print!("First disc is passable in {} (", steps);
        // for s in discs.iter().map(|d| d.passable_in_steps()) {
        // print!("{}, ", s);
        // }
        // println!(")");

        let rotateby = match steps {
            1 => discs[0].positions,
            0 => discs[0].positions - 1,
            _ => steps - 1,
        };
        // println!("Rotating by {}\n\n\n", rotateby);
        count += rotateby;
        discs = rotate_all_discs(&discs, rotateby);
    }
}


#[test]
fn test_disc_rotation() {
    let disc1 = Disc::new(4, 0);
    assert_eq!(disc1.positions, 4);
    assert_eq!(disc1.current, 0);

    let disc1 = disc1.rotate(1);
    assert_eq!(disc1.positions, 4);
    assert_eq!(disc1.current, 1);

    let disc1 = disc1.rotate(1);
    assert_eq!(disc1.positions, 4);
    assert_eq!(disc1.current, 2);

    let disc1 = disc1.rotate(1);
    assert_eq!(disc1.positions, 4);
    assert_eq!(disc1.current, 3);

    let disc1 = disc1.rotate(10);
    assert_eq!(disc1.positions, 4);
    assert_eq!(disc1.current, 1);
}

#[test]
fn test_can_pass_discs() {
    let discs = discs_for_sample();
    assert!(!can_pass_discs(&discs));

    let discs = rotate_all_discs(&discs, 5);
    // for d in &discs {
    // println!("{} of {} passable in {}",
    //  d.current,
    //  d.positions,
    //  d.passable_in_steps());
    // }
    assert!(can_pass_discs(&discs));

    let discs = vec![Disc::new(5, 4), Disc::new(20, 18), Disc::new(8, 5)];
    assert!(can_pass_discs(&discs));
}

#[test]
fn test_rotate_all() {
    let discs = vec![Disc::new(7, 2), Disc::new(6, 1)];
    let discs = rotate_all_discs(&discs, 10);
    assert_eq!(discs[0].current, 5);
    assert_eq!(discs[1].current, 5);
}

#[test]
fn test_rotate_until_passable_is_passable_now() {
    let discs = vec![Disc::new(5, 4), Disc::new(6, 4)];
    let count = rotate_until_passable(&discs);
    assert_eq!(count, 0);
}

#[test]
fn test_rotate_until_passable_is_passable_in_3() {
    let discs = vec![Disc::new(5, 1), Disc::new(6, 1)];
    let count = rotate_until_passable(&discs);
    assert_eq!(count, 3);
}

#[test]
fn test_sample() {
    let discs = discs_for_sample();
    let count = rotate_until_passable(&discs);
    assert_eq!(count, 5);
}

#[test]
fn test_passable_in_steps() {
    let discs = discs_for_sample();

    assert_eq!(discs[0].passable_in_steps(), 1);
    assert_eq!(discs[1].passable_in_steps(), 1);
}
