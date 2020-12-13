use crate::DayError;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, str::FromStr};

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

pub fn part2() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let requirements = parse_input_part2(input)?;
    let answer = solve_part2(&requirements);
    Ok(format!("The earliest time is {}", answer))
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum BusReq {
    Free,
    Bus(usize),
}

impl FromStr for BusReq {
    type Err = DayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match usize::from_str(s).map(|busnum| BusReq::Bus(busnum)) {
            Ok(bus) => Ok(bus),
            Err(e) => {
                if s == "x" {
                    Ok(BusReq::Free)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BusReqs {
    requirements: Vec<BusReq>,
}

impl Display for BusReqs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.requirements
                .iter()
                .map(|r| match r {
                    BusReq::Free => "x".to_owned(),
                    BusReq::Bus(b) => b.to_string(),
                })
                .join(",")
        )
    }
}

impl FromStr for BusReqs {
    type Err = DayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let requirements = s
            .split(",")
            .map(|x| BusReq::from_str(x))
            .collect::<Result<Vec<_>, _>>()?;
        if requirements.len() == 0 {
            Err(DayError::InputParseError(format!("No buses!")))
        } else if requirements[0] == BusReq::Free {
            Err(DayError::InputParseError(format!(
                "First bus must not be a free slot"
            )))
        } else {
            Ok(BusReqs { requirements })
        }
    }
}

fn parse_input_part2(s: &str) -> Result<BusReqs, DayError> {
    let line = s
        .lines()
        .nth(1)
        .ok_or(DayError::InputParseError(format!("Not enough lines")))?;
    BusReqs::from_str(line)
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

#[test]
fn test_parse_part2_input() {
    let input = "939
7,13,x,x,59,x,31,19
";
    let parsed = parse_input_part2(input).expect("We expect this input to parse");
    assert_eq!(
        parsed.requirements,
        vec![
            BusReq::Bus(7),
            BusReq::Bus(13),
            BusReq::Free,
            BusReq::Free,
            BusReq::Bus(59),
            BusReq::Free,
            BusReq::Bus(31),
            BusReq::Bus(19)
        ]
    );
}

#[test]
fn test_part2_with_daniels_solution() {
    let input = "939
7,13,x,x,59,x,31,19
";
    let requirements = parse_input_part2(input).expect("We expect this input to parse");
    let result = solve_part2(&requirements);

    assert_eq!(result, 1068781);
}

/// This solution borrowed from Daniel Patrick
/// with some notes by me trying to understand why it works
fn solve_part2(requirements: &BusReqs) -> i128 {
    // turn the bus requirements into a map of bus number -> bus time offset from the time we're looking for
    let buses = requirements
        .requirements
        .iter()
        .enumerate()
        .filter_map(|(i, r)| match r {
            &BusReq::Bus(b) => Some((b as i128, (b as i128 - i as i128))),
            BusReq::Free => None,
        })
        .collect::<HashMap<_, _>>();

    // least common multiple of all the time intervals involved
    let product = buses.keys().product::<i128>();

    let mut total = 0;

    // go through the map of buses
    for (divisor, remainder) in buses {
        if remainder == 0 {
            // if this is the bus on relative timestamp 0 we need do nothing
            continue;
        }

        let mut iteration = 1;
        let other_buses_time = product / divisor;

        // figure out how many times we have to go through all the other buses before we align to this one
        while iteration * other_buses_time % divisor != 1 {
            iteration += 1;
        }

        // add that to the total overall
        // this is not finding the earliest time, it's just finding a time where everything lines up
        // it stacks one bus after another, gradually aligning each one each time through the loop
        total += other_buses_time * remainder * iteration;
    }

    // and so now we have that instance that's really super massive, smash it down to size using the LCM of all the bus times
    return total % product;
}
