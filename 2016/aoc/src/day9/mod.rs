use nom::{digit, IResult};
use std::str;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Segment<'a> {
    Data(&'a [u8]),
    Expand { times: usize, source: &'a [u8] },
}

impl<'a> Segment<'a> {
    fn expanded_length(&self) -> usize {
        match self {
            &Segment::Data(d) => str::from_utf8(d).unwrap().len(),
            &Segment::Expand { times, source } => times * (str::from_utf8(source).unwrap().len()),
        }
    }

    fn fully_expanded_length(&self) -> usize {
        match self {
            &Segment::Data(b) => b.len(),
            &Segment::Expand { times, ref source } => {
                times *
                (parse_input(str::from_utf8(&source).unwrap())
                    .iter()
                    .map(|s| s.iter().fold(0, |acc, s| acc + s.fully_expanded_length()))
                    .sum::<usize>())
            }
        }
    }
}

fn segments_length(segments: &[Segment]) -> usize {
    segments.iter().fold(0, |a, s| a + s.expanded_length())
}

fn segments_length_full(segments: &[Segment]) -> usize {
    segments.iter().fold(0, |a, s| a + s.fully_expanded_length())
}

// nom parsers
named!(as_usize(&[u8]) -> usize, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
named!(capture(&[u8]) -> Segment, do_parse!(
    tag!("(") >>
    len: as_usize >>
    tag!("x") >>
    times: as_usize >>
    tag!(")") >>
    block: take!(len) >>
    (Segment::Expand { times: times, source: block })));

fn is_not_open_bracket(c: u8) -> bool {
    c != b'('
}
fn make_data(d: &[u8]) -> Result<Segment, &str> {
    Ok(Segment::Data(d))
}
named!(block(&[u8]) -> Segment, map_res!(ws!(take_while!(is_not_open_bracket)), make_data));
named!(segment(&[u8]) -> Segment, alt!(capture | block));
named!(segments(&[u8]) -> Vec<Segment>, many0!(segment));

fn parse_input(input: &str) -> Option<Vec<Segment>> {
    match segments(input.as_bytes()) {
        IResult::Done(_, segs) => Some(segs),
        _ => None,
    }
}

pub fn do_day9(input: &str) {
    let segments = parse_input(input).unwrap_or(vec![]);

    println!("Uncompressed length is {}", segments_length(&segments));
    println!("Fully uncompressed length is {}",
             segments_length_full(&segments));
}

#[test]
fn parser_test() {
    assert_eq!(capture(b"(7x6)abcdefg"),
               IResult::Done(&b""[..],
                             Segment::Expand {
                                 times: 6,
                                 source: "abcdefg".as_bytes(),
                             }));

    assert_eq!(block(b"57383("),
               IResult::Done(&b"("[..], Segment::Data(b"57383")));

    assert_eq!(segments(b"57383"),
               IResult::Done(&b""[..], vec![Segment::Data(b"57383")]));

    assert_eq!(segments(b"(2x3)abab"),
               IResult::Done(&b""[..],
                             vec![Segment::Expand {
                                      times: 3,
                                      source: b"ab",
                                  },
                                  Segment::Data(b"ab")]));
}

#[test]
fn test_parse() {
    assert_eq!(parse_input("cd(2x3)abab"),
               Some(vec![Segment::Data(b"cd"),
                         Segment::Expand {
                             times: 3,
                             source: b"ab",
                         },
                         Segment::Data(b"ab")]));
}


#[test]
fn test_expand() {
    assert_eq!(Segment::Data(b"ab").expand(),
               String::from_str("ab").unwrap());
    assert_eq!(Segment::Expand {
                       times: 2,
                       source: b"ab",
                   }
                   .expand(),
               String::from_str("abab").unwrap());
}

#[test]
fn test_expand_segments() {
    assert_eq!(expand_segments(&[Segment::Data(b"ab"), Segment::Data(b"cd")]),
               "abcd".to_owned());
}

#[test]
fn test_uncompress() {
    assert_eq!(uncompress("ADVENT"), "ADVENT".to_owned());
    assert_eq!(uncompress("A(1x5)BC"), "ABBBBBC".to_owned());
    assert_eq!(uncompress("(3x3)XYZ"), "XYZXYZXYZ".to_owned());
    assert_eq!(uncompress("A(2x2)BCD(2x2)EFC"), "ABCBCDEFEFC".to_owned());
    assert_eq!(uncompress("(6x1)(1x3)A"), "(1x3)A".to_owned());
    assert_eq!(uncompress("X(8x2)(3x3)ABCY"),
               "X(3x3)ABC(3x3)ABCY".to_owned());
}
