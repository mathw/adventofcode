use std::str::FromStr;
use nom::digit;
use std::str;

named!(pub as_u32<u32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
named!(as_i32_neg<i32>, do_parse!(
    tag!("-") >>
    theint: as_u32 >>
    (theint as i32 * -1)
));
named!(as_i32_pos<i32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
named!(pub as_i32<i32>, alt!(
    as_i32_pos |
    as_i32_neg
));

#[test]
fn test_i32() {
    let input = b"-3";
    let input2 = b"5";

    assert_done_and_eq!(as_i32(input), -3);
    assert_done_and_eq!(as_i32(input2), 5);
}
