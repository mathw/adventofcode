use nom::{IResult, alpha};
use nom_test_helpers;
use super::types::{Generator, Microchip, Element, Floor, FloorContents};
use std::str;

#[derive(Debug, PartialEq, Eq)]
enum Thing {
    Gen(Generator),
    Chip(Microchip),
}

named!(element<Element>, do_parse!(
    word: map_res!(alpha, str::from_utf8) >>
    (Element::new(word.to_owned()))
));
named!(generator<Generator>, do_parse!(
    element: ws!(element) >>
    ws!(tag!("generator")) >>
    (Generator::new(element))
));
named!(compatible_element<Element>, do_parse!(
    element: element >>
    tag!("-compatible") >>
    (element)
));
named!(microchip<Microchip>, do_parse!(
    element: ws!(compatible_element) >>
    ws!(tag!("microchip")) >>
    (Microchip::new(element))
));

fn floor_from_tag(tag: &[u8]) -> Option<Floor> {
    match tag {
        b"first" => Some(Floor::First),
        b"second" => Some(Floor::Second),
        b"third" => Some(Floor::Third),
        b"fourth" => Some(Floor::Fourth),
        _ => None,
    }
}
named!(floor_number<Floor>, map_opt!(alt!(
    tag!("first") | tag!("second") | tag!("third") | tag!("fourth")
), floor_from_tag));
named!(floor_spec<Floor>, do_parse!(
    ws!(tag!("The")) >>
    floor: ws!(floor_number) >>
    ws!(tag!("floor")) >>
    (floor)
));

named!(thing_chip<Thing>, do_parse!(
    chip: microchip >>
    (Thing::Chip(chip))
));
named!(thing_generator<Thing>, do_parse!(
    generator: generator >>
    (Thing::Gen(generator))
));
named!(a_thing<Thing>, do_parse!(
    ws!(tag!("a")) >>
    thing: alt!(thing_chip | thing_generator) >>
    (thing)
));

named!(list_sep, alt!(tag!(", and ") | tag!(", ")));
named!(list_of_things(&[u8]) -> Vec<Thing>, separated_list!(list_sep, a_thing));
named!(nothing_relevant(&[u8]) -> Vec<Thing>, do_parse!(
    tag!("nothing relevant") >>
    (Vec::new()))
);

named!(possibly_list_of_things(&[u8]) -> Vec<Thing>, alt!(
    nothing_relevant | list_of_things
));

named!(declaration<(Floor, Vec<Thing>)>, ws!(do_parse!(
    floor: ws!(floor_spec) >>
    tag!("contains") >>
    things: ws!(possibly_list_of_things) >>
    (floor, things)
)));

pub fn parse_line(line: &str) -> Option<(Floor, FloorContents)> {
    match declaration(line.as_bytes()) {
        IResult::Done(_, (floor, things)) => {
            Some((floor,
                  FloorContents::new_with_contents(things.iter().filter_map(|t| match t {
                                                       &Thing::Gen(ref g) => Some(g.clone()),
                                                       _ => None,
                                                   }),
                                                   things.iter().filter_map(|t| match t {
                                                       &Thing::Chip(ref c) => Some(c.clone()),
                                                       _ => None,
                                                   }))))
        }
        _ => None,
    }
}

#[test]
fn test_parse_element() {
    assert_done_and_eq!(element(b"ruby"), Element::new("ruby".to_owned()));
    assert_done_and_eq!(element(b"ruby-"), Element::new("ruby".to_owned()));
}

#[test]
fn test_parse_generator() {
    assert_done_and_eq!(generator(b"ruby generator"), Generator::new(Element::new("ruby".to_owned())));
    assert_error!(generator(b"ruby hosepipe"));
}

#[test]
fn test_compatible_element() {
    assert_done_and_eq!(compatible_element(b"ruby-compatible"), Element::new("ruby".to_owned()));
    assert_needed!(compatible_element(b"ruby"));
}

#[test]
fn test_microchip() {
    assert_done_and_eq!(microchip(b"ruby-compatible microchip"), Microchip::new(Element::new("ruby".to_owned())));
    assert_error!(microchip(b"hosepipe horse elephant"));
}

#[test]
fn test_floor_number() {
    assert_done_and_eq!(floor_number(b"first"), Floor::First);
    assert_error!(floor_number(b"floor"));
}

#[test]
fn test_floor_spec() {
    assert_done_and_eq!(floor_spec(b"The second floor"), Floor::Second);
}

#[test]
fn test_a_thing() {
    assert_done_and_eq!(a_thing(b"a flourine-compatible microchip"), Thing::Chip(Microchip::new(Element::new("flourine".to_owned()))));
    assert_done_and_eq!(a_thing(b"a flourine generator"), Thing::Gen(Generator::new(Element::new("flourine".to_owned()))));
}

#[test]
fn test_list() {
    assert_done_and_eq!(list_of_things(b"a ruby-compatible microchip, and a hydrogen generator"),
     vec![Thing::Chip(Microchip::new(Element::new("ruby".to_owned()))),
        Thing::Gen(Generator::new(Element::new("hydrogen".to_owned())))]);
    assert_done_and_eq!(possibly_list_of_things(b"a santa generator, and a penguin-compatible microchip"),
    vec![Thing::Gen(Generator::new(Element::new("santa".to_owned()))),
      Thing::Chip(Microchip::new(Element::new("penguin".to_owned())))]);
}

#[test]
fn parse_declaration() {
    assert_done_and_eq!(declaration(b"The third floor contains nothing relevant"), (Floor::Third, vec![]));
    assert_done_and_eq!(declaration(b"The second floor contains a santa generator, and a penguin-compatible microchip"),
     (Floor::Second, vec![Thing::Gen(Generator::new(Element::new("santa".to_owned()))),
      Thing::Chip(Microchip::new(Element::new("penguin".to_owned())))]));
}
