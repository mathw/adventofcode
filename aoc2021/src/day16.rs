use crate::day::{DayResult, PartResult};
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Packet {
    version: u8,
    body: PacketBody,
}

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    Ok(DayResult::new(
        PartResult::Success(format!(
            "Version sum is {}",
            part1(include_str!("inputs/day16.txt"))?
        )),
        PartResult::NotImplemented,
    ))
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum PacketBody {
    Literal(u64),
    Operator(Vec<Packet>),
}

fn parse_packet(bits: &[bool]) -> Option<(Packet, &[bool])> {
    println!("START Parse a packet...");
    if bits.len() < 6 {
        println!("END packet isn't long enough {}", bits.len());
        return None;
    }
    let version = decode_number(&bits[0..3]).try_into().ok()?;
    println!("Version is {}", version);
    let type_id = decode_number(&bits[3..6]);
    let (body, rem) = match type_id {
        4 => {
            println!("packet body is literal");
            let (literal, rem) = decode_literal(&bits[6..]);
            (PacketBody::Literal(literal), rem)
        }
        _ => {
            println!("packet body is operator");
            decode_operator(&bits[6..])?
        }
    };

    let packet = Packet { version, body };
    println!("END packet created {:?}", packet);
    Some((packet, rem))
}

fn decode_literal(bits: &[bool]) -> (u64, &[bool]) {
    let mut accumulated_bits = Vec::new();
    let mut bits_consumed = 0;

    for chunk in bits.chunks(5) {
        let cont = chunk[0];
        for &b in chunk[1..].into_iter() {
            accumulated_bits.push(b);
        }
        bits_consumed += 5;

        if !cont {
            break;
        }
    }

    println!("decode_literal: consumed {} bits", bits_consumed);
    (decode_number(&accumulated_bits), &bits[bits_consumed..])
}

fn decode_operator(bits: &[bool]) -> Option<(PacketBody, &[bool])> {
    println!("START decoding operator...");
    let length_type = bits[0];
    let body = &bits[1..];
    let (subpackets, rem) = match length_type {
        false => {
            let subpackets_length = decode_number(&body[0..15]) as usize;
            println!(
                "Subpackets data length is {} with remaining body length {}",
                subpackets_length,
                body.len() - 15
            );
            let boundary = 15 + subpackets_length;
            let (packets, _) = decode_packet_sequence(&body[15..boundary])?;
            (packets, &body[boundary..])
        }
        true => {
            let subpackets_count = decode_number(&body[0..11]);
            println!("Expecting {} subpackets", subpackets_count);
            decode_packet_sequence_of_length(&body[11..], subpackets_count)?
        }
    };
    Some((PacketBody::Operator(subpackets), rem))
}

fn decode_packet_sequence(bits: &[bool]) -> Option<(Vec<Packet>, &[bool])> {
    let mut packets = Vec::new();
    let mut rem = bits;

    loop {
        if let Some((packet, remainder)) = parse_packet(rem) {
            rem = remainder;
            packets.push(packet);
        } else {
            break;
        }
    }

    Some((packets, rem))
}

fn decode_packet_sequence_of_length(bits: &[bool], length: u64) -> Option<(Vec<Packet>, &[bool])> {
    let mut packets = Vec::new();
    let mut rem = bits;

    loop {
        if let Some((packet, remainder)) = parse_packet(rem) {
            rem = remainder;
            packets.push(packet);
        } else {
            break;
        }
        if packets.len() == length as usize {
            break;
        }
    }

    Some((packets, rem))
}

fn hex_char_to_binary(c: char) -> Option<Vec<bool>> {
    match c.to_ascii_uppercase() {
        '0' => Some(vec![false, false, false, false]),
        '1' => Some(vec![false, false, false, true]),
        '2' => Some(vec![false, false, true, false]),
        '3' => Some(vec![false, false, true, true]),
        '4' => Some(vec![false, true, false, false]),
        '5' => Some(vec![false, true, false, true]),
        '6' => Some(vec![false, true, true, false]),
        '7' => Some(vec![false, true, true, true]),
        '8' => Some(vec![true, false, false, false]),
        '9' => Some(vec![true, false, false, true]),
        'A' => Some(vec![true, false, true, false]),
        'B' => Some(vec![true, false, true, true]),
        'C' => Some(vec![true, true, false, false]),
        'D' => Some(vec![true, true, false, true]),
        'E' => Some(vec![true, true, true, false]),
        'F' => Some(vec![true, true, true, true]),
        _ => None,
    }
}

fn hex_string_to_binary(input: &str) -> Option<Vec<bool>> {
    Some(
        input
            .chars()
            .map(|c| hex_char_to_binary(c))
            .collect::<Option<Vec<Vec<bool>>>>()?
            .into_iter()
            .flat_map(|x| x)
            .collect(),
    )
}

fn decode_number(bits: &[bool]) -> u64 {
    let r = bits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if *b { 0b1 } else { 0b0 } << i)
        .sum();
    println!("Decoding number {} = {}", render_binary(bits), r);
    r
}

fn part1(input: &str) -> Result<u64, String> {
    let bits = hex_string_to_binary(input).ok_or(format!("Unable to parse input"))?;
    let (packet, _) = parse_packet(&bits).ok_or(format!("Unable to parse packet"))?;
    println!("{:?}", packet);
    Ok(sum_versions(&packet))
}

fn sum_versions(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.body {
            PacketBody::Literal(_) => 0,
            PacketBody::Operator(subs) => subs.into_iter().map(|s| sum_versions(&s)).sum(),
        }
}

#[test]
fn test_decode_number() {
    assert_eq!(decode_number(&vec![true, true, false]), 6);
}

#[test]
fn test_decode_literal() {
    assert_eq!(
        decode_literal(&vec![
            true, false, true, true, true, true, true, true, true, false, false, false, true,
            false, true
        ])
        .0,
        2021
    );
}

#[test]
fn test_part1_samples() {
    //assert_eq!(part1("8A004A801A8002F478").unwrap(), 16);
    assert_eq!(part1("620080001611562C8802118E34").unwrap(), 12);
    // assert_eq!(part1("C0015000016115A2E0802F182340").unwrap(), 23);
    // assert_eq!(part1("A0016C880162017C3686B18A3D4780").unwrap(), 31);
}

#[test]
fn test_parse_packet() {
    let (packet, _) = parse_packet(&hex_string_to_binary("D2FE28").unwrap()).unwrap();
    assert_eq!(packet.version, 6);
    assert_eq!(packet.body, PacketBody::Literal(2021));
}

#[test]
fn test_parse_packet2() {
    let (packet, _) = parse_packet(&hex_string_to_binary("38006F45291200").unwrap()).unwrap();
    assert_eq!(packet.version, 1);
    match packet.body {
        PacketBody::Operator(subpackets) => {
            assert_eq!(subpackets.len(), 2);
            assert_eq!(subpackets[0].body, PacketBody::Literal(10));
            assert_eq!(subpackets[1].body, PacketBody::Literal(20));
        }
        _ => assert!(false, "wrong packet type"),
    }
}

fn render_binary(bits: &[bool]) -> String {
    bits.iter().map(|b| if *b { '1' } else { '0' }).collect()
}
