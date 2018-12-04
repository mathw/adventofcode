use crate::day::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day4 {
    input: &'static str,
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            input: include_str!("input.txt"),
        }
    }
}

impl Day for Day4 {
    fn part1(&mut self, sender: &Sender<String>) {
        let events = self
            .input
            .lines()
            .map(|line| parse_entry(line))
            .collect::<Option<Vec<RawEntry>>>();
        if events.is_none() {
            sender.send("Unable to parse entries".into()).unwrap();
            return;
        }

        let events = events.unwrap();
        let sorted_events = sort_raw_entries(events);
    }

    fn part2(&mut self, sender: &Sender<String>) {}
}

fn parse_entry(source: &str) -> Option<RawEntry> {
    lazy_static! {
        static ref ENTRY_RE: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)").unwrap();
    }

    if let Some(cap) = ENTRY_RE.captures_iter(source).next() {
        let mut entry = RawEntry::new(
            u32::from_str(&cap[1]).ok()? * 10000
                + u32::from_str(&cap[2]).ok()? * 100
                + u32::from_str(&cap[3]).ok()?,
            u8::from_str(&cap[4]).ok()?,
            u8::from_str(&cap[5]).ok()?,
            parse_event(&cap[6])?,
        );

        Some(entry)
    } else {
        None
    }
}

fn parse_event(source: &str) -> Option<RawEvent> {
    lazy_static! {
        static ref BEGINSHIFT_RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        static ref SLEEP_RE: Regex = Regex::new(r"falls asleep").unwrap();
        static ref WAKE_RE: Regex = Regex::new(r"wakes up").unwrap();
    }

    if let Some(cap) = BEGINSHIFT_RE.captures_iter(source).next() {
        Some(RawEvent::BeginShift(u16::from_str(&cap[1]).ok()?))
    } else if SLEEP_RE.is_match(source) {
        Some(RawEvent::Sleep)
    } else if WAKE_RE.is_match(source) {
        Some(RawEvent::Wake)
    } else {
        None
    }
}

fn sort_raw_entries<I: IntoIterator<Item = RawEntry>>(iter: I) -> Vec<RawEntry> {
    let mut entries = iter.into_iter().collect::<Vec<_>>();
    entries.sort_unstable_by_key(|x| x.key);
    entries
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RawEntry {
    day: u32,
    hour: u8,
    minute: u8,
    event: RawEvent,
    key: u64,
}

impl RawEntry {
    fn new(day: u32, hour: u8, minute: u8, event: RawEvent) -> RawEntry {
        RawEntry {
            day,
            hour,
            minute,
            event,
            key: (day as u64) * 10000 + (hour as u64) * 100 + minute as u64,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RawEvent {
    Sleep,
    Wake,
    BeginShift(u16),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Minute {
    day: u32,
    hour: u8,
    minute: u8,
    guard: u16,
    is_asleep: bool
}

impl Minute {
    fn new(day: u32, hour: u8, minute: u8, guard: u16, is_asleep: bool) -> Minute {
        Minute { day, hour, minute, guard, is_asleep }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_raw_sleep() {
        let result = parse_entry("[1518-12-01 04:23] falls asleep").expect("This should parse");
        assert_eq!(result, RawEntry::new(15181201, 4, 23, RawEvent::Sleep));
    }
    #[test]
    fn parses_raw_wake() {
        let result = parse_entry("[1518-12-01 04:23] wakes up").expect("This should parse");
        assert_eq!(result, RawEntry::new(15181201, 4, 23, RawEvent::Wake));
    }
    #[test]
    fn parses_raw_onduty() {
        let result =
            parse_entry("[1518-12-01 04:23] Guard #45 begins shift").expect("This should parse");
        assert_eq!(
            result,
            RawEntry::new(15181201, 4, 23, RawEvent::BeginShift(45))
        );
    }
    #[test]
    fn sorts_raw_events() {
        let events = vec![
            RawEntry::new(15181201, 01, 02, RawEvent::Sleep),
            RawEntry::new(15181201, 01, 01, RawEvent::BeginShift(4)),
            RawEntry::new(15181204, 06, 06, RawEvent::Wake),
        ];
        let sorted_events = vec![
            RawEntry::new(15181201, 01, 01, RawEvent::BeginShift(4)),
            RawEntry::new(15181201, 01, 02, RawEvent::Sleep),
            RawEntry::new(15181204, 06, 06, RawEvent::Wake),
        ];

        assert_eq!(sort_raw_entries(events), sorted_events);
    }
}
