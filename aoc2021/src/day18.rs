use crate::day::{DayResult, PartResult};
use chumsky::prelude::*;
use itertools::Itertools;
use std::{collections::LinkedList, ops::Add, str::FromStr};

pub fn run() -> Result<DayResult, Box<dyn std::error::Error>> {
    let part1 = part1(include_str!("inputs/day18.txt"))?;
    let part2 = part2(include_str!("inputs/day18.txt"))?;
    Ok(DayResult::new(
        PartResult::Success(format!("The answer is {}", part1)),
        PartResult::Success(format!(
            "The largest magnitude from two numbers is {}",
            part2
        )),
    ))
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SnailfishNumber {
    values: LinkedList<SnailfishValue>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SnailfishValue {
    value: u32,
    depth: usize,
}

impl SnailfishValue {
    fn deeper(&self) -> Self {
        Self {
            value: self.value,
            depth: self.depth + 1,
        }
    }
}

impl SnailfishNumber {
    fn split(&mut self) -> bool {
        if let Some(index_to_split) = self
            .values
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value >= 10)
            .map(|(i, _)| i)
            .next()
        {
            let mut back_side = self.values.split_off(index_to_split);
            let split_num = back_side.pop_front().unwrap();
            let new_depth = split_num.depth + 1;
            self.values.push_back(SnailfishValue {
                value: split_num.value / 2,
                depth: new_depth,
            });
            self.values.push_back(SnailfishValue {
                value: (split_num.value / 2) + (split_num.value % 2),
                depth: new_depth,
            });
            self.values.append(&mut back_side);
            true
        } else {
            false
        }
    }

    fn explode(&mut self) -> bool {
        let explode_pair: Vec<(usize, SnailfishValue)> = self
            .values
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, v)| v.depth >= 4)
            .take(2)
            .collect();

        if explode_pair.len() < 2 {
            // no explosions necessary
            return false;
        }

        let left_index = explode_pair[0].0;
        let left_value = &explode_pair[0].1;
        let right_index = explode_pair[1].0;
        let right_value = &explode_pair[1].1;

        if left_index + 1 != right_index {
            panic!("Exploding pair don't have neighbouring indicies, the list is corrupted. List: {:?}", self.values);
        }

        let mut back_side = self.values.split_off(left_index);
        // drop the pair we're exploding
        back_side.pop_front();
        back_side.pop_front();

        if let Some(to_left) = self.values.back_mut() {
            // need to modify the number to the left
            to_left.value += left_value.value;
        }

        // construct new 0 element
        let new_zero = SnailfishValue {
            value: 0,
            depth: left_value.depth - 1,
        };
        self.values.push_back(new_zero);

        if let Some(to_right) = back_side.front_mut() {
            // need to modify the number to the right
            to_right.value += right_value.value;
        }

        self.values.append(&mut back_side);

        true
    }

    fn reduce(&mut self) {
        if self.explode() {
            self.reduce();
        } else {
            if self.split() {
                self.reduce();
            }
        }
    }

    fn magnitude(&self) -> u32 {
        let mut flattened = self.values.clone();
        while flattened.len() > 1 {
            // the first two deepest elements must be a pair
            let deepest = flattened.iter().map(|v| v.depth).max().unwrap();
            let deepest_two = flattened
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, v)| v.depth == deepest)
                .take(2)
                .collect::<Vec<_>>();
            let new_value = SnailfishValue {
                value: deepest_two[0].1.value * 3 + deepest_two[1].1.value * 2,
                depth: if deepest == 0 { 0 } else { deepest - 1 },
            };
            let mut back = flattened.split_off(deepest_two[0].0);
            back.pop_front();
            back.pop_front();
            flattened.push_back(new_value);
            flattened.append(&mut back);
        }

        flattened.pop_front().unwrap().value
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: Self) -> Self::Output {
        let mut values = LinkedList::new();
        for v in self
            .values
            .iter()
            .chain(rhs.values.iter())
            .map(|v| v.deeper())
        {
            values.push_back(v)
        }
        let mut sfn = SnailfishNumber { values };
        sfn.reduce();
        sfn
    }
}

impl FromStr for SnailfishNumber {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = parser()
            .parse(s)
            .map_err(|s| s.into_iter().map(|e| e.to_string()).collect::<String>())?;
        Ok(SnailfishNumber { values })
    }
}

enum ValueOrList {
    Value(SnailfishValue),
    List(LinkedList<SnailfishValue>),
}

fn parser() -> impl Parser<char, LinkedList<SnailfishValue>, Error = Simple<char>> {
    recursive(|pair| {
        let int = text::int(10)
            .map(|s: String| s.parse::<u32>().unwrap())
            .map(|i| ValueOrList::Value(SnailfishValue { value: i, depth: 0 }));
        let int_or_nested_pair = int.or(pair.map(|p| ValueOrList::List(p)));
        let inner_pair = int_or_nested_pair
            .clone()
            .then_ignore(just(','))
            .then(int_or_nested_pair)
            .map(|(l, r)| {
                let mut list = LinkedList::new();
                match l {
                    ValueOrList::Value(sv) => list.push_back(sv),
                    ValueOrList::List(l) => {
                        for mut sv in l {
                            sv.depth += 1;
                            list.push_back(sv);
                        }
                    }
                }
                match r {
                    ValueOrList::Value(sv) => list.push_back(sv),
                    ValueOrList::List(l) => {
                        for mut sv in l {
                            sv.depth += 1;
                            list.push_back(sv);
                        }
                    }
                }
                list
            });
        inner_pair.delimited_by('[', ']')
    })
}

fn part1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sfns = input
        .lines()
        .map(|l| SnailfishNumber::from_str(l))
        .collect::<Result<LinkedList<_>, _>>()?;
    let first = sfns.pop_front().ok_or(format!("No numbers were parsed"))?;
    let result = sfns.into_iter().fold(first, |a, x| a + x);
    Ok(result.magnitude())
}

fn part2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sfns = input
        .lines()
        .map(|l| SnailfishNumber::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;
    let pairs = sfns.into_iter().permutations(2);
    let magnitudes = pairs.map(|p| (p[0].clone() + p[1].clone()).magnitude());
    let answer = magnitudes.max().ok_or(format!("No numbers were parsed"))?;
    Ok(answer)
}

#[test]
fn test_parser() {
    let pair = SnailfishNumber::from_str("[1,2]").unwrap();
    assert_eq!(
        pair.values.into_iter().collect::<Vec<_>>(),
        vec![
            SnailfishValue { value: 1, depth: 0 },
            SnailfishValue { value: 2, depth: 0 }
        ]
    );
}

#[test]
fn test_split() {
    let mut must_split = SnailfishNumber::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
    must_split.split();
    assert_eq!(
        must_split,
        SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap()
    );
    must_split.split();
    assert_eq!(
        must_split,
        SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap()
    )
}

#[test]
fn test_explode() {
    let mut must_explode = SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]").unwrap();
    must_explode.explode();
    assert_eq!(
        must_explode,
        SnailfishNumber::from_str("[[[[0,9],2],3],4]").unwrap()
    );

    let mut must_explode = SnailfishNumber::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
    must_explode.explode();
    assert_eq!(
        must_explode,
        SnailfishNumber::from_str("[7,[6,[5,[7,0]]]]").unwrap()
    );

    let mut must_explode = SnailfishNumber::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();
    must_explode.explode();
    assert_eq!(
        must_explode,
        SnailfishNumber::from_str("[[6,[5,[7,0]]],3]").unwrap()
    );
}

#[test]
fn test_part1_samples() {
    fn test_sample(left: &str, right: &str, result: &str) {
        let left = SnailfishNumber::from_str(left).unwrap();
        let right = SnailfishNumber::from_str(right).unwrap();
        let result = SnailfishNumber::from_str(result).unwrap();
        assert_eq!(left + right, result);
    }

    test_sample(
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    );
}

#[test]
fn test_magnitude() {
    assert_eq!(
        SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .unwrap()
            .magnitude(),
        3488
    );
}

#[test]
fn test_part1_sample() {
    let result = part1(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    )
    .unwrap();
    assert_eq!(result, 4140);
}

#[test]
fn test_part2_sample() {
    let result = part2(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    )
    .unwrap();
    assert_eq!(result, 3993);
}
