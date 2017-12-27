use regex::Regex;
use std::str::FromStr;
use util::timed;
use std::collections::HashSet;

pub fn go() {
    let input = include_str!("input.txt");

    let (result, time) = timed(|| part1("abcdefghijklmnop", input));

    println!("[{}ms] {}", time, result);

    let (result, time) = timed(|| part2("abcdefghijklmnop", input));

    println!("[{}ms] {}", time, result);
}

fn part1(dancers: &str, moves: &str) -> String {
    let dancers = Dancers {
        dancers: dancers.chars().collect(),
    };
    let moves = parse_moves(moves);

    dancers.dance(&moves).dancers.iter().collect()
}

fn part2(dancers: &str, moves: &str) -> String {
    let moves = parse_moves(moves);

    dance_a_lot(&moves, &dancers.chars().collect(), 1_000_000_000)
        .iter()
        .collect()
}

fn dance_a_lot(
    moves: &Vec<DanceMove>,
    original_dancers: &Vec<char>,
    iterations: usize,
) -> Vec<char> {
    let mut dancers = Dancers {
        dancers: (*original_dancers).clone(),
    };

    let mut seen_before = HashSet::new();
    let mut previous_sequence = Vec::new();
    let mut last_seen_index = None;
    let mut sequence_size = None;

    for iteration in 0..iterations {
        dancers = dancers.dance(moves);
        if !seen_before.contains(&dancers.dancers) {
            seen_before.insert(dancers.dancers.clone());
            previous_sequence.push(dancers.dancers.clone());
        } else {
            let index_of_previous_sighting = previous_sequence
                .iter()
                .enumerate()
                .filter(|&(i, x)| x == &dancers.dancers)
                .next()
                .unwrap()
                .0;

            match last_seen_index {
                None => last_seen_index = Some(index_of_previous_sighting),
                Some(last_seen) => {
                    // we've hit sequence mode!
                    sequence_size = Some(previous_sequence.len());
                    break;
                }
            }
        }
        if iteration % 1000 == 0 {
            println!("After iteration {}: {:?}", iteration + 1, dancers.dancers);
        }
    }

    if let Some(ss) = sequence_size {
        println!("Detected a sequence of size {}", ss);

        let actually_required_iterations = iterations % ss;
        println!("Proceeding to dance {} times", actually_required_iterations);
        dancers = Dancers {
            dancers: (*original_dancers).clone(),
        };

        for _ in 0..actually_required_iterations {
            dancers = dancers.dance(moves);
        }
    } else {
        println!("No sequence detected");
    }

    dancers.dancers
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Debug, Clone, PartialEq)]
struct Dancers {
    dancers: Vec<char>,
}

fn parse_moves(input: &str) -> Vec<DanceMove> {
    input.split(',').filter_map(parse_move).collect()
}

fn parse_move(m: &str) -> Option<DanceMove> {
    lazy_static! {
        static ref SPIN: Regex = Regex::new(r"s(\d+)").unwrap();
        static ref EXCHANGE: Regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
        static ref PARTNER: Regex = Regex::new(r"p(.)/(.)").unwrap();
    }

    match SPIN.captures(m) {
        Some(caps) => usize::from_str(&caps[1])
            .ok()
            .and_then(|s| Some(DanceMove::Spin(s))),
        None => match EXCHANGE.captures(m) {
            Some(caps) => usize::from_str(&caps[1]).ok().and_then(|a| {
                usize::from_str(&caps[2])
                    .ok()
                    .and_then(|b| Some(DanceMove::Exchange(a, b)))
            }),
            None => match PARTNER.captures(m) {
                Some(caps) => char::from_str(&caps[1]).ok().and_then(|x| {
                    char::from_str(&caps[2])
                        .ok()
                        .and_then(|y| Some(DanceMove::Partner(x, y)))
                }),
                None => None,
            },
        },
    }
}

impl Dancers {
    fn dance<I>(&self, moves: &I) -> Dancers
    where
        I: IntoIterator<Item = DanceMove> + Clone,
    {
        (*moves)
            .clone()
            .into_iter()
            .fold(self.clone(), |d, m| d.apply_move(m))
    }

    fn apply_move(&self, m: DanceMove) -> Dancers {
        match m {
            DanceMove::Spin(s) => self.spin(s),
            DanceMove::Exchange(a, b) => self.exchange(a, b),
            DanceMove::Partner(x, y) => self.swap(x, y),
        }
    }

    fn spin(&self, s: usize) -> Dancers {
        Dancers {
            dancers: self.dancers
                .iter()
                .skip(self.dancers.len() - s)
                .take(s)
                .chain(self.dancers.iter().take(self.dancers.len() - s))
                .cloned()
                .collect(),
        }
    }

    fn exchange(&self, a: usize, b: usize) -> Dancers {
        let mut dancers = self.dancers.clone();
        let ea = dancers[a];
        let eb = dancers[b];
        dancers[b] = ea;
        dancers[a] = eb;

        Dancers { dancers: dancers }
    }

    fn swap(&self, x: char, y: char) -> Dancers {
        let px = self.dancers
            .iter()
            .enumerate()
            .filter_map(|(i, &a)| if a == x { Some(i) } else { None })
            .take(1)
            .next();
        let py = self.dancers
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b == y { Some(i) } else { None })
            .take(1)
            .next();

        match (px, py) {
            (Some(x), Some(y)) => self.exchange(x, y),
            _ => panic!("Cannot exchange {} and {} as they could not be found", x, y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin() {
        let dancers = Dancers {
            dancers: vec!['a', 'b', 'c', 'd', 'e'],
        };
        let result = dancers.spin(3);
        assert_eq!(result.dancers, vec!['c', 'd', 'e', 'a', 'b']);
    }

    #[test]
    fn test_exchange() {
        let dancers = Dancers {
            dancers: vec!['e', 'a', 'b', 'c', 'd'],
        };
        let result = dancers.exchange(3, 4);
        assert_eq!(result.dancers, vec!['e', 'a', 'b', 'd', 'c']);
    }

    #[test]
    fn test_exchange2() {
        let dancers = Dancers {
            dancers: vec!['e', 'a', 'b', 'c', 'd'],
        };
        let result = dancers.exchange(0, 2);
        assert_eq!(result.dancers, vec!['b', 'a', 'e', 'c', 'd']);
    }

    #[test]
    fn test_swap() {
        let dancers = Dancers {
            dancers: vec!['e', 'a', 'b', 'd', 'c'],
        };
        let result = dancers.swap('e', 'b');
        assert_eq!(result.dancers, vec!['b', 'a', 'e', 'd', 'c']);
    }

    #[test]
    fn test_parse_swap() {
        let r = parse_move("s7");
        assert_eq!(r, Some(DanceMove::Spin(7)));
    }

    #[test]
    fn test_parse_exchange() {
        let r = parse_move("x7/90");
        assert_eq!(r, Some(DanceMove::Exchange(7, 90)));
    }

    #[test]
    fn test_parse_partner() {
        let r = parse_move("pf/i");
        assert_eq!(r, Some(DanceMove::Partner('f', 'i')));
    }

    #[test]
    fn test_sample_part_two() {
        let moves = parse_moves("s1,x3/4,pe/b");

        let result = dance_a_lot(&moves, &vec!['a', 'b', 'c', 'd', 'e'], 2)
            .iter()
            .collect::<String>();

        assert_eq!(result, "ceadb");
    }
}
