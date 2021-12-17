use crate::day::{DayResult, PartResult};
use std::error::Error;
pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let max_y = find_launch_velocity_for_highest_y(&REAL_TARGET);
    let velocities_which_hit = count_valid_launch_velocities(&REAL_TARGET);
    Ok(DayResult::new(
        PartResult::Success(format!("Maximum y was {}", max_y)),
        PartResult::Success(format!(
            "{} velocities result in a hit",
            velocities_which_hit
        )),
    ))
}

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl TargetArea {
    fn contains(&self, pos: &Pos) -> bool {
        pos.x <= self.max_x && pos.x >= self.min_x && pos.y <= self.max_y && pos.y >= self.min_y
    }
}

static REAL_TARGET: TargetArea = TargetArea {
    min_x: 139,
    max_x: 187,
    min_y: -148,
    max_y: -89,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProbeState {
    position: Pos,
    velocity: Velocity,
}

impl ProbeState {
    fn tick(&self) -> Self {
        let new_x_pos = self.position.x + self.velocity.x;
        let new_y_pos = self.position.y + self.velocity.y;
        let new_x_velocity = if self.velocity.x < 0 {
            self.velocity.x + 1
        } else {
            if self.velocity.x > 0 {
                self.velocity.x - 1
            } else {
                0
            }
        };
        let new_y_velocity = self.velocity.y - 1;
        Self {
            position: Pos {
                x: new_x_pos,
                y: new_y_pos,
            },
            velocity: Velocity {
                x: new_x_velocity,
                y: new_y_velocity,
            },
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum RunResult {
    TooShort,
    TooLong,
    TooLow,
    Hit(i32),
}

fn run_until_hit_target_area(state: &ProbeState, target: &TargetArea) -> RunResult {
    let mut current_state = state.clone();
    let mut max_y = state.position.y;
    loop {
        if current_state.position.y > max_y {
            max_y = current_state.position.y;
        }
        if target.contains(&current_state.position) {
            return RunResult::Hit(max_y);
        } else {
            // consider whether we can ever hit the target
            if current_state.velocity.x <= 0 && current_state.position.x < target.min_x {
                return RunResult::TooShort;
            }
            if current_state.velocity.x >= 0 && current_state.position.x > target.max_x {
                return RunResult::TooLong;
            }
            if current_state.position.y < target.min_y && current_state.velocity.y < 0 {
                // we are irrevocably below the target
                return RunResult::TooLow;
            }
            // there's still hope, keep ticking
            current_state = current_state.tick();
        }
    }
}

fn count_valid_launch_velocities(target: &TargetArea) -> usize {
    let mut valid_velocities = Vec::new();
    let mut initial_velocity = Some(Velocity {
        x: 1,
        y: target.min_y,
    });

    let x_velocity_search_limit = target.max_x.abs();
    let y_velocity_search_limit = 2 * x_velocity_search_limit;

    println!(
        "Searching velocities to X {} Y {} for target area {:?}",
        x_velocity_search_limit, y_velocity_search_limit, target
    );

    loop {
        match run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: initial_velocity.clone().unwrap(),
            },
            target,
        ) {
            RunResult::Hit(_) => {
                valid_velocities.push(initial_velocity.clone().unwrap());
            }
            _ => {
                println!("{:?} does not hit", initial_velocity.clone().unwrap());
            }
        }
        initial_velocity = next_velocity(
            &initial_velocity.unwrap(),
            x_velocity_search_limit,
            y_velocity_search_limit,
        );
        if initial_velocity.is_none() {
            // search complete
            break;
        }
    }

    println!("Found valid velocities as follows: {:?}", valid_velocities);

    valid_velocities.len()
}

fn find_launch_velocity_for_highest_y(target: &TargetArea) -> i32 {
    // I'm sure there's a nice way to find a reasonable starting velocity
    let mut initial_velocity = Some(Velocity { x: 1, y: 1 });
    let mut max_y = i32::MIN;
    let x_velocity_search_limit = target.max_x.abs();
    let y_velocity_search_limit = 2 * x_velocity_search_limit;

    loop {
        match run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: initial_velocity.clone().unwrap(),
            },
            target,
        ) {
            RunResult::Hit(y) => {
                if max_y < y {
                    max_y = y;
                }
            }
            RunResult::TooLong => {}
            RunResult::TooShort => {}
            RunResult::TooLow => {}
        }
        initial_velocity = next_velocity(
            &initial_velocity.unwrap(),
            x_velocity_search_limit,
            y_velocity_search_limit,
        );
        if initial_velocity.is_none() {
            // search complete
            return max_y;
        }
    }
}

fn next_velocity(v: &Velocity, x_limit: i32, y_limit: i32) -> Option<Velocity> {
    if v.x < x_limit {
        Some(Velocity { x: v.x + 1, y: v.y })
    } else {
        if v.y < y_limit {
            Some(Velocity { x: 1, y: v.y + 1 })
        } else {
            None
        }
    }
}

#[cfg(test)]
static TEST_TARGET_AREA: TargetArea = TargetArea {
    min_x: 20,
    max_x: 30,
    min_y: -10,
    max_y: -5,
};

#[test]
fn test_part1_sample_1() {
    assert_eq!(
        run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: Velocity { x: 6, y: 2 }
            },
            &TEST_TARGET_AREA
        ),
        RunResult::Hit(3)
    );
}

#[test]
fn test_part1_sample_2() {
    assert_eq!(
        run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: Velocity { x: 6, y: 3 }
            },
            &TEST_TARGET_AREA
        ),
        RunResult::Hit(6)
    );
}

#[test]
fn test_part1_sample_3() {
    assert_eq!(
        run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: Velocity { x: 9, y: 0 }
            },
            &TEST_TARGET_AREA
        ),
        RunResult::Hit(0)
    );
}

#[test]
fn test_part1_sample_4() {
    assert_eq!(
        run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: Velocity { x: 17, y: -4 }
            },
            &TEST_TARGET_AREA
        ),
        RunResult::TooLong
    );
}

#[test]
fn test_part1_sample_5() {
    assert_eq!(
        run_until_hit_target_area(
            &ProbeState {
                position: Pos::default(),
                velocity: Velocity { x: 6, y: 9 }
            },
            &TEST_TARGET_AREA
        ),
        RunResult::Hit(45)
    );
}

#[test]
fn test_part1_sample() {
    assert_eq!(find_launch_velocity_for_highest_y(&TEST_TARGET_AREA), 45);
}

#[test]
fn test_part2_sample() {
    assert_eq!(count_valid_launch_velocities(&TEST_TARGET_AREA), 112);
}
