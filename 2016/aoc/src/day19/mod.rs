mod part2;
use self::part2::do_day19_part2;

pub fn do_day19() {
    let elves = 3005290;

    let elf = elf_circle(elves);
    println!("Out of {} elves, elf #{} has all the presents", elves, elf);

    do_day19_part2();
}



fn elf_circle(elves: usize) -> u32 {
    // initialise elf circle vector
    let mut circle = Vec::new();

    for _ in 0..elves {
        circle.push(1);
    }

    // circle is now in starting state. Begin!
    loop {
        if is_complete(&circle) {
            // return the first (only) elf with presents
            return circle.iter()
                .enumerate()
                .skip_while(|&(_, x)| *x == 0)
                .take(1)
                .next()
                .unwrap()
                .0 as u32 + 1;
        }

        // let the present-taking commence!
        for elf in 0..elves {
            // ignore elves with no presents
            if circle[elf] == 0 {
                continue;
            }
            let left_elf = elf_to_left(&circle, elf);
            take_presents(&mut circle, elf, left_elf);
        }
    }
}

fn is_complete(circle: &Vec<u32>) -> bool {
    let mut seen = false;
    for elf in circle {
        let has_presents = *elf > 0;
        if seen && has_presents {
            return false;
        }
        if has_presents {
            seen = true;
        }
    }

    return true;
}

/// Returns the index of the next elf to the left which has some presents
fn elf_to_left(circle: &Vec<u32>, elf: usize) -> usize {
    let mut to_left = if elf == circle.len() - 1 { 0 } else { elf + 1 };

    loop {
        if circle[to_left] != 0 {
            return to_left;
        }

        to_left += 1;

        if to_left >= circle.len() {
            to_left = 0;
        }
    }
}

/// Take all the presents from the takee and give them to the taker
fn take_presents(circle: &mut Vec<u32>, taker: usize, takee: usize) {
    circle[taker] += circle[takee];
    circle[takee] = 0;
}


#[test]
fn test_elf_to_left() {
    let circle = vec![1, 1, 1, 1, 1];

    assert_eq!(elf_to_left(&circle, 0), 1);
    assert_eq!(elf_to_left(&circle, 4), 0);

    let circle = vec![2, 0, 2, 0];
    assert_eq!(elf_to_left(&circle, 0), 2);
    assert_eq!(elf_to_left(&circle, 2), 0);
}

#[test]
fn test_take_presents() {
    let mut circle = vec![1, 1, 1, 1, 1];
    take_presents(&mut circle, 0, 1);
    assert_eq!(circle, vec![2, 0, 1, 1, 1]);
    take_presents(&mut circle, 2, 3);
    assert_eq!(circle, vec![2, 0, 2, 0, 1]);
    take_presents(&mut circle, 4, 0);
    assert_eq!(circle, vec![0, 0, 2, 0, 3]);
    take_presents(&mut circle, 2, 4);
    assert_eq!(circle, vec![0, 0, 5, 0, 0]);
}

#[test]
fn test_complete() {
    let mut circle = vec![1, 1, 1, 1, 1];
    take_presents(&mut circle, 0, 1);
    assert!(!is_complete(&circle));
    take_presents(&mut circle, 2, 3);
    assert!(!is_complete(&circle));
    take_presents(&mut circle, 4, 0);
    assert!(!is_complete(&circle));
    take_presents(&mut circle, 2, 4);
    assert!(is_complete(&circle));
}

#[test]
fn test_circle() {
    let r = elf_circle(5);

    assert_eq!(r, 3);
}
