use regex::Regex;
use std::str::FromStr;
use std::fmt::Debug;
use std::collections::HashMap;

pub fn do_day21(input: &str) {
    let clear = "abcdefgh";
    let instructions = input.lines().collect();
    let scrambled = scramble(clear, &instructions);

    println!("{} {}", scrambled, unscramble(&scrambled, &instructions));

    let unscrambled = unscramble("fbgdceah", &instructions);

    println!("{}", unscrambled);
}

fn scramble(src: &str, instructions: &Vec<&str>) -> String {
    let instructions = instructions.iter().map(|&i| match Command::from_str(i) {
        Ok(c) => c,
        Err(e) => panic!(e),
    });
    let mut src = src.chars().collect::<Vec<char>>();

    for i in instructions {
        i.apply_to(&mut src);
    }

    src.into_iter().collect()
}

fn unscramble(src: &str, instructions: &Vec<&str>) -> String {
    // unscrambling only appears reversible for input strings of length 8
    // because of the rule about indexes >= 4 on the "reverse based on letter"
    // operation. Therefore we can't unscramble the sample.
    // But we can be deterministic about where the instructions send each
    // letter from an 8-letter string.
    // So, build a map of how that works, and we can use that map to rebuild the original
    let sample = "abcdefgh";
    let scrambled = scramble(sample, instructions);
    let scrambled_vec = scrambled.chars().collect::<Vec<_>>();

    // build a permutation map of scrambled location -> original location
    let mut permutation_map = HashMap::new();

    for (i, c) in sample.chars().enumerate() {
        permutation_map.insert(index_of(&scrambled_vec, &c).unwrap(), i);
    }

    // apply the permutations to fix up the string
    let mut unscrambled_vec = vec!['z'; 8];
    for (i, c) in src.chars().enumerate() {
        let target_index = permutation_map.get(&i).unwrap();
        let t = unscrambled_vec.get_mut(*target_index).unwrap();
        *t = c;
    }

    unscrambled_vec.into_iter().collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command<Letter> {
    SwapPosition(usize, usize),
    SwapLetter(Letter, Letter),
    RotateLeft(usize),
    RotateRight(usize),
    RotateOnPosition(Letter),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl<Letter> Command<Letter>
    where Letter: Eq + Copy + Debug
{
    fn apply_to(&self, src: &mut Vec<Letter>) {
        #[cfg(test)]
        println!("Applying {:?} to {:?}", self, src);
        match *self {
            Command::SwapPosition(a, b) => apply_swap_position(src, a, b),
            Command::SwapLetter(a, b) => apply_swap_letter(src, a, b),
            Command::RotateLeft(steps) => apply_rotate_left(src, steps),
            Command::RotateRight(steps) => apply_rotate_right(src, steps),
            Command::RotateOnPosition(letter) => apply_rotate_on_position(src, letter),
            Command::Reverse(from, to) => apply_reverse_from_to(src, from, to),
            Command::Move(from, to) => apply_move(src, from, to),
        }
    }
}

impl FromStr for Command<char> {
    type Err = String;

    fn from_str(src: &str) -> Result<Command<char>, String> {
        lazy_static! {
            static ref SWAP_POS: Regex = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
            static ref SWAP_LETTER: Regex = Regex::new(r"swap letter (.) with letter (.)").unwrap();
            static ref ROTATE_LEFT: Regex = Regex::new(r"rotate left (\d+) steps?").unwrap();
            static ref ROTATE_RIGHT: Regex = Regex::new(r"rotate right (\d+) steps?").unwrap();
            static ref ROTATE_ON: Regex = Regex::new(r"rotate based on position of letter (.)").unwrap();
            static ref REVERSE: Regex = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
            static ref MOVE: Regex = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
        }
        if src.starts_with("swap position") {
            let caps = SWAP_POS.captures_iter(src)
                .next()
                .ok_or(format!("Couldn't match swap position for '{}'", src))?;
            let a = usize::from_str(&caps[1]).map_err(|_| format!("Parse failure converting {}", &caps[1]))?;
            let b = usize::from_str(&caps[2]).map_err(|_| format!("Parse failure converting {}", &caps[2]))?;
            Ok(Command::SwapPosition(a, b))
        } else if src.starts_with("swap letter") {
            let caps = SWAP_LETTER.captures_iter(src)
                .next()
                .ok_or(format!("Couldn't match swap letter for '{}'", src))?;
            let a = caps[1].chars().next().unwrap();
            let b = caps[2].chars().next().unwrap();
            Ok(Command::SwapLetter(a, b))
        } else if src.starts_with("rotate left") {
            let caps = ROTATE_LEFT.captures_iter(src)
                .next()
                .ok_or(format!("Couldn't match rotate left for '{}'", src))?;
            let steps = usize::from_str(&caps[1]).map_err(|_| format!("Parse failure converting {}", &caps[1]))?;
            Ok(Command::RotateLeft(steps))
        } else if src.starts_with("rotate right") {
            let caps = ROTATE_RIGHT.captures_iter(src)
                .next()
                .ok_or(format!("Couldn't match rotate right for '{}'", src))?;
            let steps = usize::from_str(&caps[1]).map_err(|_| format!("Parse failure converting {}", &caps[1]))?;
            Ok(Command::RotateRight(steps))
        } else if src.starts_with("rotate based") {
            let caps = ROTATE_ON.captures_iter(src)
                .next()
                .ok_or(format!("Couldn't match rotate on for '{}'", src))?;
            let letter = caps[1].chars().next().unwrap();
            Ok(Command::RotateOnPosition(letter))
        } else if src.starts_with("reverse positions") {
            let caps = REVERSE.captures_iter(src)
                .next()
                .ok_or(format!("Couldn't match reverse positions for '{}'", src))?;
            let from = usize::from_str(&caps[1]).map_err(|_| format!("Parse failure converting {}", &caps[1]))?;
            let to = usize::from_str(&caps[2]).map_err(|_| format!("Parse failure converting {}", &caps[2]))?;
            Ok(Command::Reverse(from, to))
        } else if src.starts_with("move") {
            let caps =
                MOVE.captures_iter(src).next().ok_or(format!("Couldn't match move for '{}'", src))?;
            let from = usize::from_str(&caps[1]).map_err(|_| format!("Parse failure converting {}", &caps[1]))?;
            let to = usize::from_str(&caps[2]).map_err(|_| format!("Parse failure converting {}", &caps[2]))?;
            Ok(Command::Move(from, to))
        } else {
            Err(format!("The source '{}' couldn't be matched to a known command prefix",
                        src))
        }
    }
}

fn apply_swap_position<T>(src: &mut Vec<T>, a: usize, b: usize) {
    assert!(src.len() > a);
    assert!(src.len() > b);

    src.swap(a, b);
}

fn apply_swap_letter<T: Eq>(src: &mut Vec<T>, a: T, b: T) {
    assert!(a != b);

    let a_pos = index_of(src, &a);
    let b_pos = index_of(src, &b);

    match (a_pos, b_pos) {
        (Some(a_pos), Some(b_pos)) => {
            apply_swap_position(src, a_pos, b_pos);
        }
        _ => {
            assert!(false);
        }
    }
}

fn apply_rotate_left<T>(src: &mut Vec<T>, steps: usize) {
    // to rotate left, fake it by rotating right!
    let right_steps = src.len() - steps;
    #[cfg(test)]
    println!("Rotating left by {} on length {} means rotate right by {}",
             steps,
             src.len(),
             right_steps);
    rotate_vec_right(src, right_steps);
}

fn apply_rotate_right<T>(src: &mut Vec<T>, steps: usize) {
    rotate_vec_right(src, steps);
}

fn reverse_vec_segment<T>(src: &mut Vec<T>, start: usize, length: usize) {
    #[cfg(test)]
    println!("Reversing from {} for {}", start, length);

    for i in 0..length / 2 {
        let j = i + start;
        let k = (length - 1 - i) + start;

        #[test]
        println!("Swapping indexes {} and {}", j, k);

        src.swap(j, k);
    }
}

fn rotate_vec_right<T>(src: &mut Vec<T>, positions: usize) {
    let length = src.len();
    reverse_vec_segment(src, 0, length);
    reverse_vec_segment(src, 0, positions);
    reverse_vec_segment(src, positions, length - positions);
}

fn index_of<T: Eq>(haystack: &Vec<T>, needle: &T) -> Option<usize> {
    haystack.iter().enumerate().filter(|&(_, x)| x == needle).next().map(|(i, _)| i)
}

fn apply_rotate_on_position<T: Eq>(src: &mut Vec<T>, letter: T) {
    let position = index_of(src, &letter);

    if let Some(position) = position {
        // rotate right based on index
        // once, then the index, then once more if the index >= 4
        let rotate_steps = if position >= 4 {
            1 + position + 1
        } else {
            1 + position
        } % src.len();

        apply_rotate_right(src, rotate_steps);
    } else {
        panic!("Asked to rotate on the position of something which wasn't in the source");
    }
}

fn apply_reverse_from_to<T>(src: &mut Vec<T>, from: usize, to: usize) {
    reverse_vec_segment(src, from, (to - from) + 1);
}

fn apply_move<T>(src: &mut Vec<T>, from: usize, to: usize) {
    assert!(to < src.len());
    assert!(from < src.len());

    let removed = src.remove(from);
    src.insert(to, removed);
}


#[test]
fn test_apply_swap_position() {
    let mut src = "abcdefg".to_owned().into_bytes();

    apply_swap_position(&mut src, 1, 3);

    assert_eq!(src,
               "adcbefg".as_bytes().iter().cloned().collect::<Vec<_>>());
}

#[test]
fn test_apply_swap_letter() {
    let mut src = "abcdefg".to_owned().into_bytes();

    apply_swap_letter(&mut src, b'a', b'f');
    assert_eq!(src, b"fbcdeag".iter().cloned().collect::<Vec<_>>());
}

#[test]
fn test_reverse_vec_segment() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    reverse_vec_segment(&mut v, 2, 4);
    assert_eq!(v, vec![1, 2, 6, 5, 4, 3, 7]);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    reverse_vec_segment(&mut v, 2, 3);
    assert_eq!(v, vec![1, 2, 5, 4, 3, 6, 7]);
}

#[test]
fn test_rotate_vec_right() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_vec_right(&mut v, 1);
    assert_eq!(v, vec![7, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_apply_rotate_left() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    apply_rotate_left(&mut v, 1);
    assert_eq!(v, vec![2, 3, 4, 5, 6, 7, 1]);
}

#[test]
fn test_apply_rotate_on_position() {
    let mut v = vec![1, 2, 3, 4, 5];
    apply_rotate_on_position(&mut v, 2);
    // should have rotated two
    assert_eq!(v, vec![4, 5, 1, 2, 3]);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    apply_rotate_on_position(&mut v, 5);
    // should have rotated 6 times as the index is >= 4
    assert_eq!(v, vec![2, 3, 4, 5, 6, 7, 1]);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    apply_rotate_on_position(&mut v, 7);
    // should have rotated 8 times as the index is >= 4
    assert_eq!(v, vec![7, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_apply_reverse_from_to() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    apply_reverse_from_to(&mut v, 2, 5);
    assert_eq!(v, vec![1, 2, 6, 5, 4, 3, 7]);

    let mut v = vec![1, 2, 3, 4, 5];
    apply_reverse_from_to(&mut v, 0, 4);
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_apply_move() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    apply_move(&mut v, 1, 4);
    assert_eq!(v, vec![1, 3, 4, 5, 2, 6, 7]);
}

#[test]
fn test_parsing() {
    assert_eq!(Command::from_str("swap position 1 with position 2").unwrap(),
               Command::SwapPosition(1, 2));
    assert_eq!(Command::from_str("swap letter a with letter b").unwrap(),
               Command::SwapLetter('a', 'b'));
    assert_eq!(Command::from_str("rotate left 4 steps").unwrap(),
               Command::RotateLeft(4));
    assert_eq!(Command::from_str("rotate right 14 steps").unwrap(),
               Command::RotateRight(14));
    assert_eq!(Command::from_str("rotate based on position of letter z").unwrap(),
               Command::RotateOnPosition('z'));
    assert_eq!(Command::from_str("reverse positions 15 through 26").unwrap(),
               Command::Reverse(15, 26));
    assert_eq!(Command::from_str("move position 5 to position 78").unwrap(),
               Command::Move(5, 78));
}

#[test]
fn test_scramble() {
    let instructions = vec!["swap position 4 with position 0",
                            "swap letter d with letter b",
                            "reverse positions 0 through 4",
                            "rotate left 1 step",
                            "move position 1 to position 4",
                            "move position 3 to position 0",
                            "rotate based on position of letter b",
                            "rotate based on position of letter d"];
    let source = "abcde";

    let result = scramble(source, &instructions);

    assert_eq!(result, "decab");
}
