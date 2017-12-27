use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;
use util::timed;

pub fn go() {
    let input = include_str!("input.txt");

    let (severity, time) = timed(|| {
        let mut firewall = parse_input(input);
        firewall.run()
    });

    println!("[{}ms] severity {}", time, severity.unwrap_or(0));

    let (delay, time) = timed(|| find_delay_for_zero_severity_passage(parse_input(input)));

    println!("[{}ms] delay {}", time, delay);
}

fn find_delay_for_zero_severity_passage(firewall_base: Firewall) -> usize {
    for delay in 0.. {
        #[cfg(test)]
        println!("Try delay {}", delay);
        let mut firewall = firewall_base.clone();
        firewall.delay(delay);
        if !firewall.run_until_caught() {
            return delay;
        }
    }

    panic!("Unable to find a delay");
}

fn parse_input(input: &str) -> Firewall {
    let mut firewall = Firewall::new();

    for line in input.lines() {
        let bits = line.split(": ").collect::<Vec<_>>();
        firewall.add_scanner(
            usize::from_str(bits[0]).unwrap(),
            usize::from_str(bits[1]).unwrap(),
        );
    }

    firewall
}

#[derive(Debug, PartialEq, Clone)]
struct Scanner {
    range: usize,
    position: usize,
    advancing: bool,
}

impl Scanner {
    fn step(&mut self) {
        #[cfg(test)]
        println!(
            "Scanner step: Range {} Position {}",
            self.range,
            self.position
        );

        if self.advancing && self.position == (self.range - 1) {
            // at the end
            // reverse
            #[cfg(test)]
            println!("Scanner at end, reversing");

            self.advancing = false;
        } else if !self.advancing && self.position == 0 {
            // at the start
            // reverse
            #[cfg(test)]
            println!("Scanner at start, reversing");

            self.advancing = true;
        }

        self.position = if self.advancing {
            self.position + 1
        } else {
            self.position - 1
        };

        #[cfg(test)]
        println!("New scanner position is {}", self.position);
    }

    fn step_by(&mut self, steps: usize) {
        let cycle_size = if self.range < 2 {
            0
        } else {
            self.range + (self.range - 2)
        };
        let remaining_steps = steps % cycle_size;

        #[cfg(test)]
        println!(
            "Asked to step {}, cycle {}, range is {}, so stepping {}",
            steps,
            cycle_size,
            self.range,
            remaining_steps
        );

        for _ in 0..remaining_steps {
            self.step();
        }
    }
}

#[derive(Clone)]
struct Firewall {
    layers: HashMap<usize, Scanner>,
    santa: usize,
}

impl Firewall {
    fn new() -> Firewall {
        Firewall {
            layers: HashMap::new(),
            santa: 0,
        }
    }

    fn add_scanner(&mut self, layer: usize, range: usize) {
        self.layers.insert(
            layer,
            Scanner {
                range: range,
                position: 0,
                advancing: true,
            },
        );
    }

    fn delay(&mut self, count: usize) {
        self.step_scanners(count);
    }

    fn advance_santa(&mut self) -> Option<usize> {
        if let Some(cost) = self.check_intersect() {
            #[cfg(test)]
            println!("Santa got caught!");
            self.santa += 1;
            return Some(cost);
        }
        #[cfg(test)]
        println!("Santa didn't get caught");
        self.santa += 1;
        None
    }

    fn tick(&mut self) -> Option<usize> {
        let penalty = self.advance_santa();
        self.step_scanners(1);
        penalty
    }

    fn step_scanners(&mut self, count: usize) {
        for (_, scanner) in self.layers.iter_mut() {
            scanner.step_by(count);
        }
    }

    fn check_intersect(&self) -> Option<usize> {
        if let Some(ref scanner) = self.layers.get(&self.santa) {
            if scanner.position == 0 {
                // caught
                return Some(self.santa * scanner.range);
            }
        }

        None
    }

    fn is_finished(&self) -> bool {
        let last_layer = self.layers.keys().max().unwrap();
        self.santa > *last_layer
    }

    fn run(&mut self) -> Option<usize> {
        self.santa = 0;

        let mut severity = None;
        while !self.is_finished() {
            match self.tick() {
                Some(penalty) => severity = Some(severity.unwrap_or(0) + penalty),
                None => {}
            }
        }

        severity
    }

    /// Run until Santa is caught, returning true if he was caught
    fn run_until_caught(&mut self) -> bool {
        self.santa = 0;

        while !self.is_finished() {
            match self.tick() {
                Some(_) => return true,
                _ => {}
            }
        }

        false
    }
}

impl fmt::Display for Firewall {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let last_layer = self.layers.keys().max().unwrap();

        for l in 0..last_layer + 1 {
            write!(f, " {:02} ", l)?;
        }

        write!(f, "\n")?;

        for _ in 0..last_layer + 1 {
            write!(f, " -- ")?;
        }

        write!(f, "\n")?;

        for l in 0..last_layer + 1 {
            match self.layers.get(&l) {
                Some(scanner) => if self.santa == l {
                    if scanner.position == 0 {
                        write!(f, "(!!)")?;
                    } else {
                        write!(f, "(  )")?;
                    }
                } else {
                    write!(f, "    ")?;
                },
                None => if self.santa == l {
                    write!(f, "(  )")?
                } else {
                    write!(f, "    ")?
                },
            }
        }
        write!(f, "\n")?;

        for l in 0..last_layer + 1 {
            match self.layers.get(&l) {
                Some(scanner) => write!(f, "[{:02}]", scanner.position)?,
                None => write!(f, "    ")?,
            }
        }
        write!(f, "\n")?;

        for l in 0..last_layer + 1 {
            match self.layers.get(&l) {
                Some(scanner) => write!(f, "[{:02}]", scanner.range)?,
                None => write!(f, "    ")?,
            }
        }
        write!(f, "\n")?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"0: 3
1: 2
4: 4
6: 4";
        let mut firewall = parse_input(input);

        let severity = firewall.run();

        assert_eq!(severity, Some(24));
    }

    #[test]
    fn test_sample_part_two() {
        let input = r"0: 3
1: 2
4: 4
6: 4";
        let firewall = parse_input(input);
        let delay = find_delay_for_zero_severity_passage(firewall);

        assert_eq!(delay, 10);
    }

    #[test]
    fn test_sample_delayed() {
        let input = r"0: 3
1: 2
4: 4
6: 4";
        let mut firewall = parse_input(input);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        firewall.delay(1);
        println!("{}", firewall);
        assert_eq!(firewall.run(), None);
    }

    #[test]
    fn test_scanner_step() {
        let mut scanner = Scanner {
            position: 0,
            range: 3,
            advancing: true,
        };

        scanner.step();
        assert_eq!(scanner.position, 1);
        scanner.step();
        assert_eq!(scanner.position, 2);
        scanner.step();
        assert_eq!(scanner.position, 1);
        scanner.step();
        assert_eq!(scanner.position, 0);
        scanner.step();
        assert_eq!(scanner.position, 1);
        scanner.step();
        assert_eq!(scanner.position, 2);
    }

    #[test]
    fn test_scanner_step_2() {
        let mut scanner = Scanner {
            position: 0,
            range: 2,
            advancing: true,
        };

        scanner.step();
        assert_eq!(scanner.position, 1, "First step is to 1");
        scanner.step();
        assert_eq!(scanner.position, 0, "Second step is to 0");
        scanner.step();
        assert_eq!(scanner.position, 1, "Third step is to 1");
        scanner.step();
        assert_eq!(scanner.position, 0, "Fourth step is to 0");
    }
}
