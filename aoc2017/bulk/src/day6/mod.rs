use std::str::FromStr;
use std::collections::HashSet;
use util::timed_repeatedly;

pub fn go(reps: usize) {
    let input = parse_input(include_str!("input.txt"));

    let ((part1, part2), time) = timed_repeatedly(reps, || {
        let mut buckets = input.clone();
        explore_loop(&mut buckets)
    });
    println!("[{}ms] {} steps until loop, {} steps in loop",
             time,
             part1,
             part2);
}

fn parse_input(input: &str) -> Vec<u32> {
    input.split_whitespace().filter_map(|x| u32::from_str(x).ok()).collect()
}

fn explore_loop(buckets: &mut Vec<u32>) -> (usize, usize) {
    let mut loop_started = false;
    let mut steps_until_repeat = 0;
    let mut steps_in_loop = 0;
    let mut states = HashSet::new();
    states.insert(buckets.clone());
    let mut first_repeated_state = buckets.clone();

    loop {
        let largest_index = largest_bucket_index(&buckets);

        redistribute(buckets, largest_index);

        if loop_started {
            steps_in_loop += 1;
            if *buckets == first_repeated_state {
                break;
            }
        } else {
            steps_until_repeat += 1;
            if states.contains(buckets) {
                loop_started = true;
                first_repeated_state = buckets.clone();
            }
        }
        states.insert(buckets.clone());
    }

    (steps_until_repeat, steps_in_loop)
}

/// Conducts a single redistribute step
fn redistribute(buckets: &mut Vec<u32>, source: usize) {
    // take everything out of the source bucket
    let mut available = buckets[source].clone();
    buckets[source] = 0;

    let num_buckets = buckets.len();

    let mut current_bucket = next_bucket(source, num_buckets);
    while available > 0 {
        buckets[current_bucket] += 1;
        available -= 1;
        current_bucket = next_bucket(current_bucket, num_buckets);
    }
}

fn largest_bucket_index(buckets: &Vec<u32>) -> usize {
    let mut largest = 0;
    let mut largest_index = 0;
    // find largest bucket
    for (i, &x) in buckets.iter().enumerate() {
        if x > largest {
            largest = x;
            largest_index = i;
        }
    }
    largest_index
}

/// Given the current bucket, and the number of buckets, return the 0-index of the next bucket.
fn next_bucket(current: usize, length: usize) -> usize {
    if current + 1 == length {
        0
    } else {
        current + 1
    }
}

#[cfg(test)]
mod tests {
    mod redistribute {
        use super::super::*;

        #[test]
        fn single_run() {
            let mut buckets = vec![0, 2, 7, 0];
            redistribute(&mut buckets, 2);
            assert_eq!(buckets, vec![2, 4, 1, 2]);
        }

        #[test]
        fn full_run() {
            let mut buckets = vec![0, 2, 7, 0];
            let (until_loop, in_loop) = explore_loop(&mut buckets);
            assert_eq!(until_loop, 5, "Steps until loop");
            assert_eq!(in_loop, 4, "Steps in loop");
        }
    }
}
