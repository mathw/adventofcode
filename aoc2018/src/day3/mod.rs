use crate::day::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::usize;

pub struct Day3 {
    input: &'static str,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            input: include_str!("input.txt"),
        }
    }
}

impl Day for Day3 {
    fn part1(&mut self, sender: &Sender<String>) {
        let claims = self
            .input
            .lines()
            .map(|line| Claim::from_str(line))
            .collect::<Vec<_>>();
        if claims.iter().any(|c| c.is_err()) {
            sender.send(format!("Error parsing claims")).unwrap();
            return;
        }

        sender
            .send(format!("Parsed {} claims", claims.len()))
            .unwrap();

        let claims = claims.into_iter().map(|m| m.unwrap()).collect::<Vec<_>>();

        let max_x = claims.iter().map(|m| m.right()).max().unwrap_or(0);
        let max_y = claims.iter().map(|m| m.bottom()).max().unwrap_or(0);

        sender
            .send(format!("Max claimed X: {}, Y: {}", max_x, max_y))
            .unwrap();

        let mut fabric = Fabric::new(1000, 1000);
        for claim in claims {
            fabric.add_claim(&claim);
        }

        let result = fabric.area_over_claims(2);

        sender
            .send(format!(
                "{}sq in of fabric are claimed at least twice",
                result
            ))
            .unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let claims = self
            .input
            .lines()
            .map(|line| Claim::from_str(line))
            .collect::<Vec<_>>();
        if claims.iter().any(|c| c.is_err()) {
            sender.send(format!("Error parsing claims")).unwrap();
            return;
        }

        sender
            .send(format!("Parsed {} claims", claims.len()))
            .unwrap();

        let claims = claims.into_iter().map(|m| m.unwrap()).collect::<Vec<_>>();

        let result = find_non_overlapping_claims(&claims);

        sender
            .send(format!("Non-overlapping claims: {:?}", result))
            .unwrap();
    }
}

trait Dimensioned {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

trait Positioned {
    fn top(&self) -> usize;
    fn left(&self) -> usize;
    fn bottom(&self) -> usize;
    fn right(&self) -> usize;
}

trait Identified<T> {
    fn id(&self) -> T;
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Rect {
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl Rect {
    fn intersects(&self, other: &Rect) -> bool {
        !(self.is_above(other)
            || self.is_left(other)
            || other.is_above(self)
            || other.is_left(self))
    }

    fn is_above(&self, other: &Rect) -> bool {
        self.bottom() <= other.top
    }

    fn is_left(&self, other: &Rect) -> bool {
        self.right() <= other.left
    }
}

impl Positioned for Rect {
    fn top(&self) -> usize {
        self.top
    }
    fn left(&self) -> usize {
        self.left
    }

    fn bottom(&self) -> usize {
        self.top + self.height
    }

    fn right(&self) -> usize {
        self.left + self.width
    }
}

impl Dimensioned for Rect {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Claim {
    id: u32,
    rect: Rect,
}

impl Claim {
    fn intersects(&self, other: &Claim) -> bool {
        self.rect.intersects(&other.rect)
    }
}

impl Positioned for Claim {
    fn top(&self) -> usize {
        self.rect.top()
    }
    fn left(&self) -> usize {
        self.rect.left()
    }

    fn bottom(&self) -> usize {
        self.rect.bottom()
    }

    fn right(&self) -> usize {
        self.rect.right()
    }
}

impl Dimensioned for Claim {
    fn height(&self) -> usize {
        self.rect.height()
    }

    fn width(&self) -> usize {
        self.rect.width()
    }
}

impl Identified<u32> for Claim {
    fn id(&self) -> u32 {
        self.id
    }
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(source: &str) -> Result<Claim, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let mut iter = re.captures_iter(source);
        if let Some(cap) = iter.next() {
            Ok(Claim {
                id: u32::from_str(&cap[1]).expect("If ID won't parse the regex is broken"),
                rect: Rect {
                    left: usize::from_str(&cap[2])
                        .expect("If left won't parse the regex is broken"),
                    top: usize::from_str(&cap[3]).expect("If top won't parse the regex is broken"),
                    width: usize::from_str(&cap[4])
                        .expect("If width won't parse the regex is broken"),
                    height: usize::from_str(&cap[5])
                        .expect("If height won't parse the regex is broken"),
                },
            })
        } else {
            Err(format!(
                "\"{}\" does not appear to be a valid claim",
                source
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Range {
    start: usize,
    end: usize,
    claims: usize,
    claimed_by: Vec<u32>,
}

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range {
            start,
            end,
            claims: 0,
            claimed_by: vec![],
        }
    }

    fn claim(&self, start: usize, end: usize, claim_id: u32) -> Option<Vec<Range>> {
        #[cfg(test)]
        println!(
            "Range::claim(): {}, {} into {}, {}",
            start, end, self.start, self.end
        );
        if self.start > start || self.end < end {
            return None;
        }

        let mut new_claimed_by = self.claimed_by.clone();
        new_claimed_by.push(claim_id);

        let mut ranges = Vec::new();
        if start > self.start {
            let new = Range {
                start: self.start,
                end: start,
                claims: self.claims,
                claimed_by: self.claimed_by.clone(),
            };
            #[cfg(test)]
            println!("Creating before range {:?}", new);
            ranges.push(new);
        }

        let new = Range {
            start: start,
            end: end,
            claims: self.claims + 1,
            claimed_by: new_claimed_by.clone(),
        };
        #[cfg(test)]
        println!("Creating mid range {:?}", new);
        ranges.push(new);

        if end < self.end {
            let new = Range {
                start: end,
                end: self.end,
                claims: self.claims,
                claimed_by: self.claimed_by.clone(),
            };
            #[cfg(test)]
            println!("Creating after range {:?}", new);
            ranges.push(new);
        }

        Some(ranges)
    }

    fn contains(&self, x: usize) -> bool {
        self.start <= x && self.end > x
    }

    fn overlaps(&self, left: usize, right: usize) -> bool {
        if self.contains(left) {
            true
        } else if right > self.start && right < self.end {
            true
        } else if left < self.start && right >= self.end {
            true
        } else {
            false
        }
    }

    fn end(&self) -> usize {
        self.end
    }

    fn start(&self) -> usize {
        self.start
    }

    fn size(&self) -> usize {
        self.end - self.start
    }

    fn claims(&self) -> usize {
        self.claims
    }

    fn claimed_by(&self) -> Vec<u32> {
        self.claimed_by.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Fabric {
    rows: Vec<Vec<Range>>,
}

impl Fabric {
    fn new(width: usize, height: usize) -> Fabric {
        let mut rows = Vec::new();

        for _ in 0..height {
            rows.push(vec![Range::new(0, width)]);
        }

        Fabric { rows }
    }

    fn add_claim<C: Positioned + Dimensioned + Debug + Identified<u32>>(&mut self, claim: &C) {
        #[cfg(test)]
        println!(
            "\n\nAdding claim @ {},{}: {}x{}",
            claim.left(),
            claim.top(),
            claim.width(),
            claim.height()
        );
        for (row_index, row) in self
            .rows
            .iter_mut()
            .enumerate()
            .skip(claim.top())
            .take(claim.height())
        {
            #[cfg(test)]
            {
                println!("\n== Row {}: {:?}", row_index, row);
                println!(
                    "Find affected ranges overlapping {}, {}",
                    claim.left(),
                    claim.right()
                );
            }
            let affected_ranges = row
                .iter()
                .enumerate()
                .filter(|(_, r)| r.overlaps(claim.left(), claim.right()))
                .collect::<Vec<_>>();

            let mut new_ranges = Vec::new();
            let mut affected_index_start = usize::MAX;
            let mut affected_index_end = 0;

            #[cfg(test)]
            println!("{} affected ranges", affected_ranges.len());

            for (index, affected_range) in affected_ranges {
                #[cfg(test)]
                println!("\n= Affected range #{}: {:?}", index, affected_range);
                affected_index_start = usize::min(index, affected_index_start);
                affected_index_end = usize::max(index, affected_index_end);

                let overlap_start = usize::max(affected_range.start(), claim.left());
                let overlap_end = usize::min(affected_range.end(), claim.right());

                if overlap_start == overlap_end {
                    // this was a bug before
                    panic!("Zero-length subrange can never happen");
                }

                #[cfg(test)]
                println!(
                    "Overlap start: {}, overlap end: {}",
                    overlap_start, overlap_end
                );

                if let Some(mut ranges) =
                    affected_range.claim(overlap_start, overlap_end, claim.id())
                {
                    new_ranges.append(&mut ranges);
                }
            }

            let _: Vec<_> = row
                .splice(
                    affected_index_start..affected_index_end + 1,
                    new_ranges.into_iter(),
                )
                .collect();

            #[cfg(test)]
            println!("Modified row: {:?}", row);
        }
    }

    fn area_over_claims(&self, claims: usize) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|range| range.claims() >= claims)
                    .map(|range| range.size())
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.claims {
            0 => '.',
            1 => 'O',
            _ => 'X',
        };

        for _ in self.start..self.end {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl Display for Fabric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            for range in row {
                write!(f, "{}", range)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn find_non_overlapping_claims(claims: &Vec<Claim>) -> Vec<u32> {
    let mut candidates = HashSet::new();

    for claim in claims {
        candidates.insert(claim.id);
    }

    for outer in claims {
        for inner in claims {
            if outer != inner && outer.intersects(inner) {
                candidates.remove(&outer.id());
                candidates.remove(&inner.id());
            }
        }
    }

    candidates.into_iter().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_from_string_valid() {
        let result = Claim::from_str("#2 @ 3,1: 4x5");

        assert_eq!(
            result,
            Ok(Claim {
                id: 2,
                rect: Rect {
                    left: 3,
                    top: 1,
                    width: 4,
                    height: 5
                }
            })
        );
    }

    #[test]
    fn claim_middle_of_range_get_three_ranges() {
        let range = Range::new(0, 6);

        let ranges = range.claim(2, 4, 1);

        assert_eq!(
            ranges,
            Some(vec![
                Range {
                    start: 0,
                    end: 2,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 2,
                    end: 4,
                    claims: 1,
                    claimed_by: vec![1]
                },
                Range {
                    start: 4,
                    end: 6,
                    claims: 0,
                    claimed_by: vec![]
                }
            ])
        );
    }

    #[test]
    fn claim_in_claim() {
        let mut fabric = Fabric::new(8, 1);
        fabric.add_claim(&Claim::from_str("#1 @ 1,0: 6x1").unwrap());
        fabric.add_claim(&Claim::from_str("#2 @ 2,0: 4x1").unwrap());

        assert_eq!(fabric.to_string(), ".OXXXXO.\n".to_owned());
    }

    #[test]
    fn render_fabric_empty() {
        let fabric = Fabric::new(4, 4);

        let result = fabric.to_string();

        assert_eq!(
            result,
            r"....
....
....
....
"
            .to_owned()
        );
    }

    #[test]
    fn fabric_claim_range() {
        let mut fabric = Fabric::new(4, 4);
        fabric.add_claim(&Claim::from_str("#4 @ 0,0: 1x1").unwrap());
        let result = fabric.to_string();

        assert_eq!(
            result,
            r"O...
....
....
....
"
        );
    }

    #[test]
    fn fabric_claim_ranges() {
        let mut fabric = Fabric::new(4, 4);
        fabric.add_claim(&Claim::from_str("#4 @ 0,0: 2x2").unwrap());
        fabric.add_claim(&Claim::from_str("#4 @ 0,1: 1x2").unwrap());
        let result = fabric.to_string();

        assert_eq!(
            result,
            r"OO..
XO..
O...
....
"
        );
    }

    #[test]
    fn overlaps() {
        let range = Range::new(2, 5);
        assert_eq!(true, range.overlaps(2, 5), "Overlaps identical range");
        assert_eq!(
            true,
            range.overlaps(1, 3),
            "Overlaps range over left boundary"
        );
        assert_eq!(
            true,
            range.overlaps(3, 6),
            "Overlaps range over right boundary"
        );
        assert_eq!(
            false,
            range.overlaps(0, 2),
            "Doesn't overlap touching range on the left"
        );
        assert_eq!(
            false,
            range.overlaps(5, 6),
            "Doesn't overlap touching range on the right"
        );
        assert_eq!(true, range.overlaps(3, 4), "Overlaps subrange");
        assert_eq!(
            true,
            Range::new(1, 3).overlaps(0, 4),
            "1, 3 range overlaps 0, 4"
        );
    }

    #[test]
    fn contains() {
        let range = Range::new(2, 5);
        assert_eq!(true, range.contains(2), "contains left boundary");
        assert_eq!(false, range.contains(5), "doesn't countain right boundary");
        assert_eq!(true, range.contains(3), "contains middle");
        assert_eq!(false, range.contains(1), "doesn't contain left neighbour");
        assert_eq!(false, range.contains(6), "doesn't contain right neighbour");
        assert_eq!(false, range.contains(0), "2, 5 doesn't contain 0");
    }

    #[test]
    fn fabric_area_claimed_twice() {
        let mut fabric = Fabric::new(4, 4);
        fabric.add_claim(&Claim::from_str("#4 @ 0,0: 2x2").unwrap());
        fabric.add_claim(&Claim::from_str("#4 @ 0,1: 1x2").unwrap());
        let result = fabric.area_over_claims(2);

        assert_eq!(result, 1);
    }

    #[test]
    fn part_one_example() {
        let mut fabric = Fabric::new(8, 8);
        fabric.add_claim(&Claim::from_str("#1 @ 1,3: 4x4").unwrap());
        fabric.add_claim(&Claim::from_str("#2 @ 3,1: 4x4").unwrap());
        fabric.add_claim(&Claim::from_str("#3 @ 5,5: 2x2").unwrap());
        let result = fabric.area_over_claims(2);

        assert_eq!(result, 4);
        let first_row = (&fabric.rows).iter().nth(0).unwrap();
        assert_eq!(
            first_row,
            &vec![Range {
                start: 0,
                end: 8,
                claims: 0,
                claimed_by: vec![]
            }]
        );
        let second_row = (&fabric.rows).iter().nth(1).unwrap();
        assert_eq!(
            second_row,
            &vec![
                Range {
                    start: 0,
                    end: 3,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 3,
                    end: 7,
                    claims: 1,
                    claimed_by: vec![2]
                },
                Range {
                    start: 7,
                    end: 8,
                    claims: 0,
                    claimed_by: vec![]
                }
            ]
        );
        let third_row = (&fabric.rows).iter().nth(2).unwrap();
        assert_eq!(
            third_row,
            &vec![
                Range {
                    start: 0,
                    end: 3,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 3,
                    end: 7,
                    claims: 1,
                    claimed_by: vec![2]
                },
                Range {
                    start: 7,
                    end: 8,
                    claims: 0,
                    claimed_by: vec![]
                }
            ]
        );
        let fourth_row = (&fabric.rows).iter().nth(3).unwrap();
        assert_eq!(
            fourth_row,
            &vec![
                Range {
                    start: 0,
                    end: 1,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 1,
                    end: 3,
                    claims: 1,
                    claimed_by: vec![1]
                },
                Range {
                    start: 3,
                    end: 5,
                    claims: 2,
                    claimed_by: vec![1, 2]
                },
                Range {
                    start: 5,
                    end: 7,
                    claims: 1,
                    claimed_by: vec![2]
                },
                Range {
                    start: 7,
                    end: 8,
                    claims: 0,
                    claimed_by: vec![]
                }
            ]
        );
        let fifth_row = (&fabric.rows).iter().nth(4).unwrap();
        assert_eq!(
            fifth_row,
            &vec![
                Range {
                    start: 0,
                    end: 1,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 1,
                    end: 3,
                    claims: 1,
                    claimed_by: vec![1]
                },
                Range {
                    start: 3,
                    end: 5,
                    claims: 2,
                    claimed_by: vec![1, 2]
                },
                Range {
                    start: 5,
                    end: 7,
                    claims: 1,
                    claimed_by: vec![2]
                },
                Range {
                    start: 7,
                    end: 8,
                    claims: 0,
                    claimed_by: vec![]
                }
            ],
            "fifth row"
        );
        let sixth_row = (&fabric.rows).iter().nth(5).unwrap();
        assert_eq!(
            sixth_row,
            &vec![
                Range {
                    start: 0,
                    end: 1,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 1,
                    end: 5,
                    claims: 1,
                    claimed_by: vec![1]
                },
                Range {
                    start: 5,
                    end: 7,
                    claims: 1,
                    claimed_by: vec![3]
                },
                Range {
                    start: 7,
                    end: 8,
                    claims: 0,
                    claimed_by: vec![]
                }
            ],
            "sixth row"
        );
        let seventh_row = (&fabric.rows).iter().nth(6).unwrap();
        assert_eq!(
            seventh_row,
            &vec![
                Range {
                    start: 0,
                    end: 1,
                    claims: 0,
                    claimed_by: vec![]
                },
                Range {
                    start: 1,
                    end: 5,
                    claims: 1,
                    claimed_by: vec![1]
                },
                Range {
                    start: 5,
                    end: 7,
                    claims: 1,
                    claimed_by: vec![3]
                },
                Range {
                    start: 7,
                    end: 8,
                    claims: 0,
                    claimed_by: vec![]
                }
            ],
            "seventh row"
        );
        assert_eq!(
            fabric.to_string(),
            r"........
...OOOO.
...OOOO.
.OOXXOO.
.OOXXOO.
.OOOOOO.
.OOOOOO.
........
"
            .to_owned()
        );
    }

    #[test]
    fn third_range_over_two_ranges() {
        let mut fabric = Fabric::new(8, 8);
        fabric.add_claim(&Claim::from_str("#1 @ 1,1: 4x4").unwrap());
        fabric.add_claim(&Claim::from_str("#1 @ 2,2: 2x2").unwrap());
        assert_eq!(
            fabric.to_string(),
            r"........
.OOOO...
.OXXO...
.OXXO...
.OOOO...
........
........
........
"
            .to_owned(),
            "After second range"
        );

        fabric.add_claim(&Claim::from_str("#1 @ 1,1: 2x2").unwrap());

        assert_eq!(
            fabric.to_string(),
            r"........
.XXOO...
.XXXO...
.OXXO...
.OOOO...
........
........
........
"
            .to_owned(),
            "After third range"
        );
        assert_eq!(7, fabric.area_over_claims(2), "After third range");

        fabric.add_claim(&Claim::from_str("#1 @ 1,1: 2x2").unwrap());

        assert_eq!(
            fabric.to_string(),
            r"........
.XXOO...
.XXXO...
.OXXO...
.OOOO...
........
........
........
"
            .to_owned(),
            "After fourth range"
        );
        assert_eq!(7, fabric.area_over_claims(2), "After fourth range");

        fabric.add_claim(&Claim::from_str("#1 @ 0,0: 4x4").unwrap());

        assert_eq!(
            fabric.to_string(),
            r"OOOO....
OXXXO...
OXXXO...
OXXXO...
.OOOO...
........
........
........
"
            .to_owned(),
            "After fifth range"
        );
        assert_eq!(9, fabric.area_over_claims(2), "After fifth range");
    }
    #[test]
    fn part_two_example() {
        let claims = vec![
            Claim::from_str("#1 @ 1,3: 4x4").unwrap(),
            Claim::from_str("#2 @ 3,1: 4x4").unwrap(),
            Claim::from_str("#3 @ 5,5: 2x2").unwrap(),
        ];

        assert_eq!(find_non_overlapping_claims(&claims), vec![3]);
    }
}
