pub struct Circle {
    marbles: Vec<u32>,
    current_marble_index: usize,
    highest_marble_value: u32,
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            marbles: vec![0],
            current_marble_index: 0,
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

    pub fn with_new_marble(&self) -> (Circle, u32) {
        let new_marble_value = self.highest_marble_value + 1;

        if new_marble_value % 23 == 0 {
            // crazy stuff now happens

            // the marble 7 marbles counter-clockwise is removed

            // #[cfg(test)]
            // println!("New marble value is {}! Fancy things!", new_marble_value);

            let removed_marble_index = if self.current_marble_index >= 7 {
                self.current_marble_index - 7
            } else {
                self.marbles.len() - (7 - self.current_marble_index)
            };

            let removed_marble_value = self.marbles[removed_marble_index];

            let mut new_marbles = self.marbles.clone();
            let _: Vec<_> = new_marbles
                .splice(removed_marble_index..removed_marble_index + 1, vec![])
                .collect();

            (
                Circle {
                    marbles: new_marbles,
                    current_marble_index: removed_marble_index,
                    highest_marble_value: new_marble_value,
                },
                new_marble_value + removed_marble_value,
            )
        } else {
            let location = self.find_insert_location();

            let mut new_marbles = self.marbles.clone();
            let _: Vec<_> = new_marbles
                .splice(location..location, vec![new_marble_value])
                .collect();

            (
                Circle {
                    marbles: new_marbles,
                    current_marble_index: location,
                    highest_marble_value: new_marble_value,
                },
                0,
            )
        }
    }
}

use std::fmt;
impl fmt::Display for Circle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (index, marble) in self.marbles.iter().enumerate() {
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

#[test]
fn find_insert_location_new_circle() {
    let circle = Circle::new();

    assert_eq!(circle.find_insert_location(), 1);
}

#[test]
fn find_insert_location_circle_of_two() {
    let circle = Circle {
        marbles: vec![0, 1],
        current_marble_index: 1,
        highest_marble_value: 1,
    };

    assert_eq!(circle.find_insert_location(), 1);
}

#[test]
fn find_insert_location_circle_of_three() {
    let circle = Circle {
        marbles: vec![0, 2, 1],
        current_marble_index: 1,
        highest_marble_value: 2,
    };

    assert_eq!(circle.find_insert_location(), 3);
}

#[test]
fn find_insert_location_circle_of_four() {
    let circle = Circle {
        marbles: vec![0, 2, 1, 3],
        current_marble_index: 3,
        highest_marble_value: 3,
    };

    assert_eq!(circle.find_insert_location(), 1);
}

#[test]
fn add_marbles() {
    let starting = Circle::new();
    let (first, _) = starting.with_new_marble();
    let (second, _) = first.with_new_marble();
    let (third, _) = second.with_new_marble();
    let (fourth, _) = third.with_new_marble();

    assert_eq!(fourth.marbles, vec![0, 4, 2, 1, 3]);
    assert_eq!(fourth.current_marble_index, 1);
}

#[test]
fn add_twenty_three_marbles() {
    let mut circle = Circle::new();
    let mut score: u32 = 0;

    println!("{}", circle);

    for _ in 0..23 {
        let (new_circle, new_score) = circle.with_new_marble();
        circle = new_circle;
        score = new_score;

        println!("{}", circle);
    }

    assert_eq!(score, 32);
}
