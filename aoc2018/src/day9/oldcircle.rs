use std::collections::HashMap;
use std::rc::Rc;
use super::marble::Marble;

pub struct Circle {
    marbles: Rc<Box<Marble>>,
    highest_marble_value: u32,
}
enum Renumber {
    Increase,
    Decrease,
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            marbles: Rc::new(Box::new(Marble::new(0)))
            highest_marble_value: 0,
        }
    }

    fn find_insert_location(&self) -> usize {
        let mut target = self.current_marble_index + 2;

        if target > self.marbles.len() {
            target -= self.marbles.len();
        }

        target
    }

    fn renumber_marbles(&mut self, from_index: usize, action: Renumber) {
        let mut marbles = HashMap::new();
        for (index, value) in self.marbles.iter() {
            if *index >= from_index {
                match action {
                    Renumber::Increase => marbles.insert(index + 1, *value),
                    Renumber::Decrease => marbles.insert(index - 1, *value),
                };
            } else {
                marbles.insert(*index, *value);
            }
        }
        self.marbles = marbles;
    }

    pub fn add_new_marble(&mut self) -> u32 {
        let new_marble_value = self.highest_marble_value + 1;

        if new_marble_value % 23 == 0 {
            // crazy stuff now happens

            // the marble 7 marbles counter-clockwise is removed

            #[cfg(test)]
            println!(
                "New marble value is {}! Fancy things!\n{}",
                new_marble_value, self
            );

            let removed_marble_index = if self.current_marble_index >= 7 {
                self.current_marble_index - 7
            } else {
                self.marbles.len() - (7 - self.current_marble_index)
            };

            let removed_marble_value = self.marbles[&removed_marble_index];

            self.marbles.remove(&removed_marble_index);
            self.renumber_marbles(removed_marble_index, Renumber::Decrease);

            self.current_marble_index = removed_marble_index;
            self.highest_marble_value = new_marble_value;

            new_marble_value + removed_marble_value
        } else {
            let location = self.find_insert_location();

            self.renumber_marbles(location, Renumber::Increase);
            self.marbles.insert(location, new_marble_value);

            self.current_marble_index = location;
            self.highest_marble_value = new_marble_value;

            0
        }
    }
}

use std::fmt;
impl fmt::Display for Circle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut marbles: Vec<u32> = self.marbles.values().cloned().collect();
        marbles.sort();

        for (index, marble) in marbles.iter().enumerate() {
            if index == self.current_marble_index {
                write!(fmt, " ({}) ", marble)?;
            } else {
                write!(fmt, " {} ", marble)?;
            }
        }
        writeln!(fmt, "")?;
        Ok(())
    }
}

#[cfg(test)]
fn make_marbles<I: IntoIterator<Item = u32>>(i: I) -> HashMap<usize, u32> {
    i.into_iter().enumerate().collect()
}

#[test]
fn find_insert_location_new_circle() {
    let circle = Circle::new();

    assert_eq!(circle.find_insert_location(), 1);
}

#[test]
fn find_insert_location_circle_of_two() {
    let circle = Circle {
        marbles: make_marbles(vec![0, 1]),
        current_marble_index: 1,
        highest_marble_value: 1,
    };

    assert_eq!(circle.find_insert_location(), 1);
}

#[test]
fn find_insert_location_circle_of_three() {
    let circle = Circle {
        marbles: make_marbles(vec![0, 2, 1]),
        current_marble_index: 1,
        highest_marble_value: 2,
    };

    assert_eq!(circle.find_insert_location(), 3);
}

#[test]
fn find_insert_location_circle_of_four() {
    let circle = Circle {
        marbles: make_marbles(vec![0, 2, 1, 3]),
        current_marble_index: 3,
        highest_marble_value: 3,
    };

    assert_eq!(circle.find_insert_location(), 1);
}

#[test]
fn add_marbles() {
    let mut circle = Circle::new();
    circle.add_new_marble();
    circle.add_new_marble();
    circle.add_new_marble();
    circle.add_new_marble();

    assert_eq!(circle.marbles, make_marbles(vec![0, 4, 2, 1, 3]));
    assert_eq!(circle.current_marble_index, 1);
}

#[test]
fn add_twenty_three_marbles() {
    let mut circle = Circle::new();
    let mut score: u32 = 0;

    println!("{}", circle);

    for _ in 0..23 {
        score = circle.add_new_marble();

        println!("{}", circle);
    }

    assert_eq!(score, 32);
}
