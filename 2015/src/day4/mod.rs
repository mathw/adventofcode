use md5;

pub fn run() {
    let input = "iwrupvqb";

    let one = partone(input);

    println!("{}", one);

    let two = parttwo(input);
    println!("{}", two);
}

fn partone(input: &str) -> u32 {
    let mut current = 0;

    while !has_five_zeroes(md5_hex(format!("{}{}", input, current)).as_ref()) {
        current += 1;
    }

    current
}

fn parttwo(input: &str) -> u32 {
    let mut current = 0;

    while !has_six_zeroes(md5_hex(format!("{}{}", input, current)).as_ref()) {
        current += 1;
    }

    current
}

fn md5_hex<T: AsRef<[u8]>>(input: T) -> String {
    format!("{:x}", md5::compute(input))
}

fn has_zeroes(count: usize, input: &str) -> bool {
    if input.len() < count {
        false
    } else {
        input[..count].chars().all(|x| x == '0')
    }
}

fn has_five_zeroes(input: &str) -> bool {
    has_zeroes(5, input)
}

fn has_six_zeroes(input: &str) -> bool {
    has_zeroes(6, input)
}

#[test]
fn has_five_zeroes_too_short() {
    assert_eq!(has_five_zeroes("00"), false);
}

#[test]
fn has_five_zeroes_doesnt() {
    assert_eq!(has_five_zeroes("000028324234872"), false);
}

#[test]
fn has_five_zeroes_does() {
    assert!(has_five_zeroes("00000"));
    assert!(has_five_zeroes("0000098324"));
}

#[test]
fn test_partone() {
    assert_eq!(partone("abcdef"), 609043);
}