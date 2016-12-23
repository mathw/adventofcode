fn get_puzzle_input() -> Vec<bool> {
    vec![true, true, false, true, true, true, true, false, false, true, true, false, true, true,
         true, false, true]
}

fn expand(input: Vec<bool>) -> Vec<bool> {
    let mut b = input.iter()
        .map(|&x| match x {
            true => false,
            false => true,
        })
        .collect::<Vec<_>>();
    b.reverse();

    let mut r = input.into_iter().collect::<Vec<_>>();
    r.push(false);
    r.append(&mut b);
    r
}

fn checksum(input: Vec<bool>) -> Vec<bool> {
    if input.len() % 2 == 1 {
        return input;
    }

    checksum(input.chunks(2).map(|chunk| chunk[0] == chunk[1]).collect())
}

fn expand_to_length(input: Vec<bool>, target: usize) -> Vec<bool> {
    let expanded = expand(input);
    if expanded.len() >= target {
        expanded
    } else {
        expand_to_length(expanded, target)
    }
}

fn crop_to_length<T>(input: Vec<T>, length: usize) -> Vec<T> {
    if input.len() == length {
        input
    } else if input.len() < length {
        panic!("Can't crop to less than the length of the input")
    } else {
        input.into_iter().take(length).collect()
    }
}

pub fn do_day16() {
    let input = get_puzzle_input();
    let data = crop_to_length(expand_to_length(input.clone(), 272), 272);
    let csum = checksum(data);

    print!("Checksum is: ");
    for bit in csum {
        print!("{}", if bit { '1' } else { '0' });
    }
    println!();

    let part2_length = 35651584;
    let data = crop_to_length(expand_to_length(input, part2_length), part2_length);
    let csum = checksum(data);

    print!("Checksum is: ");
    for bit in csum {
        print!("{}", if bit { '1' } else { '0' });
    }
    println!();
}


#[test]
fn test_expand() {
    assert_eq!(expand(vec![true]), vec![true, false, false]);
    assert_eq!(expand(vec![false]), vec![false, false, true]);
    assert_eq!(expand(vec![true, true, true, true, true]),
               vec![true, true, true, true, true, false, false, false, false, false, false]);
    assert_eq!(expand(vec![true, true, true, true, false, false, false, false, true, false,
                           true, false]),
               vec![true, true, true, true, false, false, false, false, true, false, true, false,
                    false, true, false, true, false, true, true, true, true, false, false, false,
                    false]);
}

#[test]
fn test_checksum() {
    assert_eq!(checksum(vec![true, true, false, false, true, false, true, true, false, true,
                             false, false]),
               vec![true, false, false]);
}

#[test]
fn test_expand_to_length() {
    assert_eq!(expand_to_length(vec![true, false, false, false, false], 20),
               vec![true, false, false, false, false, false, true, true, true, true, false,
                    false, true, false, false, false, false, true, true, true, true, true, false]);
}

#[test]
fn test_crop_to_length() {
    assert_eq!(crop_to_length(vec![true, false, false], 2),
               vec![true, false]);
}
