use std::str::FromStr;
use std::ops::Mul;
use util::timed;

pub fn go() {
    let input = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113";

    let (result, time) = timed(|| {
        part1(
            input
                .split(',')
                .filter_map(|x| usize::from_str(x).ok())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    });

    println!("[{}ms] hash is {}", time, result);

    let (result, time) = timed(|| part2(input.chars().map(|c| c as u8)));

    println!("[{}ms] hash is {}", time, result);
}

fn part1(input: &[usize]) -> u16 {
    let start = (0..256).collect::<Vec<u16>>();

    hash(start.as_slice(), input)
}

fn part2<I>(input: I) -> String
where
    I: IntoIterator<Item = u8>,
{
    fullhash(input)
}

fn fullhash<I>(input: I) -> String
where
    I: IntoIterator<Item = u8>,
{
    render_hash(
        dense_hash(sparse_hash(input.into_iter().map(|i| i as usize)).as_slice()).as_slice(),
    )
}

fn sparse_hash<I>(input: I) -> Vec<u8>
where
    I: IntoIterator<Item = usize>,
{
    let mut lengths = input.into_iter().collect::<Vec<_>>();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    let mut working = (0..256).map(|x| x as u8).collect::<Vec<_>>();
    let mut start = 0;
    let mut skip = 0;

    for _ in 0..64 {
        hash_round(&mut working, lengths.as_slice(), &mut start, &mut skip);
    }

    working
}

fn dense_hash(sparse_hash: &[u8]) -> Vec<u8> {
    sparse_hash
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, item| acc ^ item))
        .collect()
}

fn render_hash(dense_hash: &[u8]) -> String {
    dense_hash.iter().map(|n| format!("{:02x}", n)).collect()
}

fn hash_round<T>(list: &mut [T], lengths: &[usize], start: &mut usize, skip: &mut usize)
where
    T: Copy + Mul<Output = T>,
{
    for &length in lengths {
        circular_reverse(list, *start, length);
        *start += length + *skip;
        *skip += 1;
    }
}

fn hash<T>(list: &[T], lengths: &[usize]) -> T
where
    T: Copy + Mul<Output = T>,
{
    let mut result = list.iter().cloned().collect::<Vec<_>>();
    let mut start = 0;
    let mut skip = 0;

    for &length in lengths {
        circular_reverse(&mut result, start, length);
        start += length + skip;
        skip += 1;
    }


    result[0] * result[1]
}

/// Reverse a segment of `list` from `start` for `length` units, treating
/// the list as a circle.
fn circular_reverse<T: Copy>(list: &mut [T], start: usize, length: usize) {
    let mut index1;
    let mut index2;

    for i in 0..length / 2 {
        index1 = (start + i) % list.len();
        index2 = (start + length - i - 1) % list.len();

        #[cfg(test)]
        println!("i = {}, index1 = {}, index2 = {}", i, index1, index2);

        swap(list, index1, index2);
    }
}

/// Swap elements in `list` at `a` and `b` in place.
fn swap<T: Copy>(list: &mut [T], a: usize, b: usize) {
    let temp = list[a];
    list[a] = list[b];
    list[b] = temp;
}

#[cfg(test)]
mod tests {
    mod reverse {
        use super::super::circular_reverse;

        #[test]
        fn reverse_length_one() {
            let mut list = vec![0, 1, 2, 3, 4];

            circular_reverse(&mut list, 1, 1);
            assert_eq!(list, vec![0, 1, 2, 3, 4]);

            circular_reverse(&mut list, 4, 1);
            assert_eq!(list, vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn reverse_entire_list() {
            let mut list = vec![0, 1, 2, 3, 4];

            circular_reverse(&mut list, 0, 5);
            assert_eq!(list, vec![4, 3, 2, 1, 0]);
        }

        #[test]
        fn reverse_over_list_boundary() {
            let mut list = vec![0, 1, 2, 3, 4];

            circular_reverse(&mut list, 3, 3);
            assert_eq!(list, vec![3, 1, 2, 0, 4]);
        }
    }

    mod hash {
        use super::super::hash;

        #[test]
        fn sample_hash() {
            let list = vec![0, 1, 2, 3, 4];
            let lengths = vec![3, 4, 1, 5];

            let result = hash(&list, &lengths);

            assert_eq!(result, 12);
        }
    }

    mod fullhash {
        use super::super::fullhash;

        #[test]
        fn sample_one() {
            assert_eq!(
                fullhash("".chars().map(|c| c as u8)),
                "a2582a3a0e66e6e86e3812dcb672a272".to_owned()
            );
        }

        #[test]
        fn sample_two() {
            assert_eq!(
                fullhash("AoC 2017".chars().map(|c| c as u8)),
                "33efeb34ea91902bb2f59c9920caa6cd".to_owned()
            );
        }

        #[test]
        fn sample_three() {
            assert_eq!(
                fullhash("1,2,3".chars().map(|c| c as u8)),
                "3efbe78a8d82f29979031a4aa0b16a9d".to_owned()
            );
        }

        #[test]
        fn sample_four() {
            assert_eq!(
                fullhash("1,2,4".chars().map(|c| c as u8)),
                "63960835bcdc130f0b66d7ff4f6a5a8e".to_owned()
            );
        }
    }
}
