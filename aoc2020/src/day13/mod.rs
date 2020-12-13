use crate::DayError;
use std::str::FromStr;

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let (waiting_time, buses) = parse_input_part1(input)?;
    let (first_bus, bus_waiting_time) =
        find_earliest_bus(waiting_time, &buses).ok_or_else(|| DayError::NoSolutionFoundError)?;
    Ok(format!(
        "Bus {} leaves first with a waiting time of {} minutes leading to the answer {}",
        first_bus,
        bus_waiting_time,
        first_bus * bus_waiting_time
    ))
}

fn parse_input_part1(s: &str) -> Result<(usize, Vec<usize>), DayError> {
    let mut lines = s.lines();
    let minutes_to_wait = lines
        .next()
        .map(|l| usize::from_str(l))
        .ok_or_else(|| DayError::InputParseError(format!("Couldn't parse first line")))??;
    let buses_in_service = lines
        .next()
        .map(|l| {
            l.split(',')
                .filter(|x| *x != "x")
                .map(|x| usize::from_str(x))
                .collect::<Result<Vec<_>, _>>()
        })
        .ok_or_else(|| DayError::InputParseError(format!("No second line")))??;
    Ok((minutes_to_wait, buses_in_service))
}

fn find_earliest_bus(min_wait: usize, bus_intervals: &Vec<usize>) -> Option<(usize, usize)> {
    let mut bus_times_to_wait = bus_intervals
        .iter()
        .cloned()
        .map(|bus| (bus, bus - (min_wait % bus)))
        .collect::<Vec<_>>();
    bus_times_to_wait.sort_by_key(|x| x.1);
    bus_times_to_wait.get(0).map(|x| *x)
}

#[test]
fn test_parse_input_part1() {
    let input = "939
7,13,x,x,59,x,31,19
";
    let parsed = parse_input_part1(input).expect("We expect this input to parse");
    assert_eq!(parsed, (939, vec![7, 13, 59, 31, 19]));
}

#[test]
fn test_part1_sample() {
    let answer =
        find_earliest_bus(939, &vec![7, 13, 59, 31, 19]).expect("Should be able to find a bus");
    assert_eq!(answer, (59, 5));
}
