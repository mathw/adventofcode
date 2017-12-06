use std::str::FromStr;
use std::collections::HashSet;
use util::timed_repeatedly;

pub fn go(reps: usize) {
    let input = parse_input(include_str!("input.txt"));

    let (result, time) = timed_repeatedly(reps, || part1(&input));
    println!("[{}ms] {} steps", time, result);

    let (result, time) = timed_repeatedly(reps, || part2(&input));
    println!("[{}ms] {} steps in loop", time, result);
}

fn part1(input: &Vec<u32>) -> usize {
    let mut buckets = input.clone();
    redistribute_until_repeated(&mut buckets)
}

fn part2(input: &Vec<u32>) -> usize {
    let mut buckets = input.clone();
    find_redistribute_loop_size(&mut buckets)
}

fn parse_input(input: &str) -> Vec<u32> {
    input.split_whitespace().filter_map(|x| u32::from_str(x).ok()).collect()
}

fn redistribute_until_repeated(buckets: &mut Vec<u32>) -> usize {
    let mut steps = 0;
    let mut states = HashSet::new();
    states.insert(buckets.clone());

    loop {
        steps += 1;
        let mut largest = 0;
        let mut largest_index = 0;
        for (i, &x) in buckets.iter().enumerate() {
            if x > largest {
                largest = x;
                largest_index = i;
            }
        }
        #[cfg(test)]
        println!("{} steps, largest bucket is {} at index {}",
                 steps,
                 largest,
                 largest_index);
        redistribute(buckets, largest_index);

        if states.contains(buckets) {
            break;
        }
        states.insert(buckets.clone());
    }

    steps
}

fn find_redistribute_loop_size(buckets: &mut Vec<u32>) -> usize {
    let mut loop_started = false;
    let mut steps = 0;
    let mut states = HashSet::new();
    states.insert(buckets.clone());
    let mut first_repeated_state = buckets.clone();

    loop {
        let mut largest = 0;
        let mut largest_index = 0;
        for (i, &x) in buckets.iter().enumerate() {
            if x > largest {
                largest = x;
                largest_index = i;
            }
        }
        #[cfg(test)]
        println!("{} steps, largest bucket is {} at index {}",
                 steps,
                 largest,
                 largest_index);
        redistribute(buckets, largest_index);

        if loop_started {
            steps += 1;
            if *buckets == first_repeated_state {
                break;
            }
        } else {
            if states.contains(buckets) {
                loop_started = true;
                first_repeated_state = buckets.clone();
            }
        }
        states.insert(buckets.clone());
    }

    steps
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
            let count = redistribute_until_repeated(&mut buckets);
            assert_eq!(count, 5);
        }

        #[test]
        fn loop_size() {
            let mut buckets = vec![0, 2, 7, 0];
            let count = find_redistribute_loop_size(&mut buckets);
            assert_eq!(count, 4);
        }
    }
}