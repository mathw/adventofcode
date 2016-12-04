mod input;

use regex::Regex;
use itertools::Itertools;
use std::cmp::Ordering;
use self::input::get_input;

#[derive(Debug, Eq, PartialEq)]
pub struct RoomIdentifier<'a> {
    pub encrypted_name: &'a str,
    pub sector_id: u32,
    pub checksum: &'a str,
}

impl<'a> RoomIdentifier<'a> {
    pub fn validate(&self) -> bool {
        calculate_checksum(self.encrypted_name) == self.checksum
    }

    pub fn decrypt_name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| if c == '-' {
                ' '
            } else {
                shift_char(c, self.sector_id)
            })
            .collect()
    }
}

fn shift_char(c: char, amount: u32) -> char {
    fn char_to_letternum(c: char) -> u8 {
        c as u8 - 'a' as u8
    }
    fn letternum_to_char(n: u8) -> char {
        char::from(n + 'a' as u8)
    }
    let actual_shift = amount % 26;
    let lowercase = c.to_lowercase().next().unwrap();
    let numeric = char_to_letternum(lowercase) as u32;
    let shifted = numeric + actual_shift;
    let new_number = shifted % 26;
    letternum_to_char(new_number as u8)
}

fn name_sort(a: &str, b: &str) -> Ordering {
    if a.len() < b.len() {
        Ordering::Greater
    } else if a.len() > b.len() {
        Ordering::Less
    } else {
        a.partial_cmp(b).unwrap()
    }
}

fn calculate_checksum(name: &str) -> String {
    let mut name_letters = name.chars().filter(|c| *c != '-').collect::<Vec<_>>();
    name_letters.sort_by(|a, b| a.cmp(b));
    // now name_letters is all the letters in order, we need to group the same letters together
    let mut groups = name_letters.into_iter()
        .group_by(|c| c.clone())
        .into_iter()
        .map(|(_, g)| g.collect::<String>())
        .collect::<Vec<String>>();

    groups.sort_by(|a, b| name_sort(a, b));
    groups.into_iter().map(|g| g.chars().next().unwrap()).take(5).collect()
}

/// Parse a room identifier of format encrypted_name-sector_id[checksum]
pub fn parse_room(input: &str) -> Option<RoomIdentifier> {
    lazy_static! {
        static ref ROOM_REGEX: Regex = Regex::new(r"^(.+)-(\d+)\[(.+)\]").unwrap();
    }

    let cap = ROOM_REGEX.captures_iter(input).next();
    match cap {
        None => None,
        Some(cap) => {
            match (cap.at(1),
                   match cap.at(2) {
                       Some(id) => {
                           match id.parse::<u32>() {
                               Ok(i) => Some(i),
                               _ => None,
                           }
                       }
                       _ => None,
                   },
                   cap.at(3)) {
                (Some(encname), Some(secid), Some(checksum)) => {
                    Some(RoomIdentifier {
                        encrypted_name: encname,
                        sector_id: secid,
                        checksum: checksum,
                    })
                }
                _ => None,
            }
        }
    }
}

pub fn do_dayfour() {
    let rooms = get_input().into_iter().filter_map(|i| parse_room(i));

    let valid_rooms = rooms.filter(|r| r.validate()).collect::<Vec<_>>();

    let sum = valid_rooms.iter().fold(0, |c, r| c + r.sector_id);

    println!("The sum of the sector IDs of all the valid rooms is {}",
             sum);

    let npo = valid_rooms.iter().filter_map(|r| if r.decrypt_name().starts_with("n") {
        Some(r)
    } else {
        None
    });

    for r in npo {
        println!("North pole objects in sector {} {}",
                 r.sector_id,
                 r.decrypt_name());
    }
}


#[test]
fn test_parse_room() {
    fn check_parse_room(input: &str, expected: Option<RoomIdentifier>) {
        assert_eq!(parse_room(input), expected);
    }
    let inputs = vec![("aaaaa-bbb-z-y-x-123[abxyz]",
                       Some(RoomIdentifier {
                           encrypted_name: "aaaaa-bbb-z-y-x",
                           sector_id: 123,
                           checksum: "abxyz",
                       })),
                      ("a-b-c-d-e-f-g-h-987[abcde]",
                       Some(RoomIdentifier {
                           encrypted_name: "a-b-c-d-e-f-g-h",
                           sector_id: 987,
                           checksum: "abcde",
                       })),
                      ("not-a-real-room-404[oarel]",
                       Some(RoomIdentifier {
                           encrypted_name: "not-a-real-room",
                           sector_id: 404,
                           checksum: "oarel",
                       })),
                      ("totally-real-room-200[decoy]",
                       Some(RoomIdentifier {
                           encrypted_name: "totally-real-room",
                           sector_id: 200,
                           checksum: "decoy",
                       }))];

    for (i, e) in inputs {
        check_parse_room(i, e);
    }
}


#[test]
pub fn test_name_sort() {
    fn check_name_sort(a: &str, b: &str, expected: Ordering) {
        assert_eq!(name_sort(a, b), expected);
    }

    check_name_sort("a", "b", Ordering::Less);
    check_name_sort("a", "a", Ordering::Equal);
    check_name_sort("b", "a", Ordering::Greater);
    check_name_sort("bb", "a", Ordering::Less);
    check_name_sort("bb", "c", Ordering::Less);
    check_name_sort("a", "bb", Ordering::Greater);
}

#[test]
pub fn test_calculate_checksum() {
    fn check(name: &str, checksum: &str) {
        assert_eq!(calculate_checksum(name), checksum);
    }

    check("a", "a");
    check("ab", "ab");
    check("abb", "ba");
    check("a-b-c-d-e", "abcde");
    check("abcdefghi", "abcde");
}

#[test]
pub fn test_shift() {
    assert_eq!(shift_char('q', 343), 'v');
    assert_eq!(shift_char('z', 343), 'e');
    assert_eq!(shift_char('m', 343), 'r');
    assert_eq!(shift_char('t', 343), 'y');
}

#[test]
pub fn test_decrypt() {
    let ri = RoomIdentifier {
        encrypted_name: "qzmt-zixmtkozy-ivhz",
        sector_id: 343,
        checksum: "abede",
    };

    assert_eq!(ri.decrypt_name(), "very encrypted name");
}
