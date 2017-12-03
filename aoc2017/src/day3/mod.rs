use std::time::Instant;
use util::asmillis::AsMillis;
use util::repeatedly;

pub fn go(reps: usize) {
    // input is a 1-indexed point number
    let input = 325489;

    let timer1 = Instant::now();
    let work1 = || get_coords_to(input - 1).manhattan_distance_to_origin();
    let part1 = repeatedly(reps, work1);
    println!("[{}ms] Distance is {}", timer1.elapsed().as_millis(), part1);
}

/// Calculates the x, y coordinates for the point `i`, indexed from 0
fn get_coords_to(i: usize) -> Point {
    // Algorithm from https://stackoverflow.com/a/3715915/241544
    let i = i as i32;
    let segment_length = (i as f64).sqrt().round() as i32;
    let steps_to_diagonal = (segment_length.pow(2) - i).abs() - segment_length;
    let calc = |l| {
        (l + segment_length.pow(2) - i - (segment_length % 2)) as f64 * 0.5 *
        (-1i32.pow(segment_length as u32) as f64)
    };
    let x = calc(steps_to_diagonal);
    let y = calc(-steps_to_diagonal);
    Point {
        x: x as i32,
        y: y as i32,
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance_to_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}