use crate::day::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day4 {
    guards: HashMap<u16, Guard>,
}

impl Day4 {
    pub fn new() -> Option<Day4> {
        let input = include_str!("input.txt");
        let guards = Day4::prepare(input)?;
        Some(Day4 { guards })
    }

    fn prepare(input: &str) -> Option<HashMap<u16, Guard>> {
        let events = input
            .lines()
            .map(|line| parse_entry(line))
            .collect::<Option<Vec<RawEntry>>>();
        if events.is_none() {
            return None;
        }

        let events = events.unwrap();
        let sorted_events = sort_raw_entries(events);

        let mut guards = HashMap::new();

        let mut current_guard_id = 0;
        let mut current_sleep_start_hour = 0;
        let mut current_sleep_start_minute = 0;

        for entry in sorted_events {
            match entry.event {
                RawEvent::BeginShift(guard_id) => current_guard_id = guard_id,
                RawEvent::Sleep => {
                    current_sleep_start_hour = entry.hour;
                    current_sleep_start_minute = entry.minute
                }
                RawEvent::Wake => {
                    let guard = guards
                        .entry(current_guard_id)
                        .or_insert(Guard::new(current_guard_id));
                    let sleep_duration = (entry.hour * 60 + entry.minute)
                        - (current_sleep_start_hour * 60 + current_sleep_start_minute);
                    guard.add_sleeping_minutes(current_sleep_start_minute, sleep_duration);
                }
            }
        }

        Some(guards)
    }
}

impl Day for Day4 {
    fn part1(&mut self, sender: &Sender<String>) {
        let mut sleepiest_guard = self.guards.values().nth(0).unwrap();
        for guard in self.guards.values() {
            if guard.total_minutes_asleep > sleepiest_guard.total_minutes_asleep {
                sleepiest_guard = guard;
            }
        }

        sender
            .send(format!(
                "Sleepiest guard #{} for {} minutes most often at {} ({})",
                sleepiest_guard.id,
                sleepiest_guard.total_minutes_asleep,
                sleepiest_guard.minute_most_often_asleep(),
                sleepiest_guard.id * sleepiest_guard.minute_most_often_asleep() as u16
            ))
            .unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let mut max_minute_count = 0;
        let mut max_minute = 0;
        let mut max_minute_count_guard_id = 0;

        for guard in self.guards.values() {
            let count = guard.minute_most_often_asleep_count();
            if max_minute_count < count {
                max_minute_count = count;
                max_minute = guard.minute_most_often_asleep();
                max_minute_count_guard_id = guard.id;
            }
        }

        sender
            .send(format!(
                "Guard #{} was asleep {} times in minute {} ({})",
                max_minute_count_guard_id,
                max_minute_count,
                max_minute,
                max_minute_count_guard_id as usize * max_minute as usize
            ))
            .unwrap();
    }
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
struct Guard {
    id: u16,
    total_minutes_asleep: u32,
    minutes_asleep: HashMap<u8, usize>,
}

impl Guard {
    fn new(id: u16) -> Guard {
        Guard {
            id: id,
            total_minutes_asleep: 0,
            minutes_asleep: HashMap::new(),
        }
    }

    fn add_sleeping_minutes(&mut self, start_minute: u8, duration: u8) {
        self.total_minutes_asleep += duration as u32;
        for minute in start_minute..(start_minute + duration) {
            let entry = self.minutes_asleep.entry(minute % 60).or_insert(0);
            *entry += 1;
        }
    }

    fn minute_most_often_asleep(&self) -> u8 {
        let mut minute = 0;
        let mut minute_times = 0;

        for (&m, &t) in self.minutes_asleep.iter() {
            if t > minute_times {
                minute_times = t;
                minute = m;
            }
        }

        minute
    }

    fn minute_most_often_asleep_count(&self) -> usize {
        self.minutes_asleep[&self.minute_most_often_asleep()]
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
