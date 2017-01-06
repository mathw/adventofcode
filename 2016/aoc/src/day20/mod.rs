use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn do_day20(input: &str) {
    let ranges = input.lines()
        .map(|l| Range::from_str(l).unwrap())
        .collect::<Vec<Range>>();
    let rangeset = RangeSet::from_ranges(ranges);

    println!("Lowest gap is {}", rangeset.lowest_gap());
    println!("{} are not included in the ranges",
             rangeset.how_many_not_included(0, 4294967295));
}

/// An inclusive range structure
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Range {
    from: u64,
    to: u64,
}

impl FromStr for Range {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Range, &'static str> {
        let split = s.split('-').collect::<Vec<_>>();
        if split.len() < 2 {
            Err("Not a valid range")
        } else {
            let start = u64::from_str(split[0]);
            let end = u64::from_str(split[1]);

            match (start, end) {
                (Ok(start), Ok(end)) => {
                    Ok(Range {
                        from: start,
                        to: end,
                    })
                }
                _ => Err("Not a valid range"),
            }
        }
    }
}

struct RangeSet {
    ranges: HashMap<u64, Range>,
}

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet { ranges: HashMap::new() }
    }

    /// Add a range to the set.
    /// Makes an assumption that ranges are always added in ascending order of start
    fn add_range(&mut self, range: Range) {
        let mut sorted_keys = self.ranges.keys().cloned().collect::<Vec<_>>();
        sorted_keys.sort();
        let previous_or_matching_start =
            sorted_keys.iter().take_while(|&k| *k <= range.from).cloned().last();

        match previous_or_matching_start {
            Some(start) => {
                let possibly_containing_range = self.ranges.get(&start).unwrap().clone();

                if range.from > possibly_containing_range.to + 1 {
                    self.ranges.insert(range.from, range);
                    return;
                }

                if possibly_containing_range.to >= range.to {
                    // do nothing - this range is already covered
                    return;
                }

                // otherwise we need to modify this range to include the new range's to-extent
                let new_range = Range {
                    from: possibly_containing_range.from,
                    to: range.to,
                };
                self.ranges.insert(new_range.from, new_range);
            }
            None => {
                self.ranges.insert(range.from, range);
            }
        }
    }

    fn from_ranges(ranges: Vec<Range>) -> RangeSet {
        let mut rangeset = RangeSet::new();

        // sort the ranges
        let mut ranges = ranges.clone();
        ranges.sort_by(|a, b| match a.from.cmp(&b.from) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.to.cmp(&b.to),
        });

        println!("Range 0: {:?}", ranges.get(0));
        println!("Range 1: {:?}", ranges.get(1));
        println!("Range 2: {:?}", ranges.get(2));
        println!("Range 3: {:?}", ranges.get(3));

        #[cfg(test)]
        println!("Sorted ranges to add are {:?}", ranges);

        for range in ranges {
            rangeset.add_range(range);
        }
        rangeset
    }

    /// find the lowest number which is not covered by any range in the set
    fn lowest_gap(&self) -> u64 {
        if self.ranges.len() == 0 {
            return 0;
        }

        match self.ranges.get(&0) {
            Some(r) => {
                // we cover 0, so we need to find the end of this first range
                r.to + 1
            }
            None => {
                // we don't cover 0, so our first gap is actually 0
                0
            }
        }
    }

    fn how_many_not_included(&self, from: u64, to: u64) -> u64 {
        if from == to {
            return 0;
        }

        let mut count = 0;

        let mut sorted_keys = self.ranges.keys().cloned().collect::<Vec<_>>();
        sorted_keys.sort();

        if sorted_keys.len() == 0 {
            return (to + 1) - from;
        }

        let mut previous_range: Option<Range> = None;

        for key in sorted_keys {
            let current_range = self.ranges.get(&key).unwrap();

            match previous_range {
                None => count = count + current_range.from,
                Some(previous_range) => count = count + current_range.from - previous_range.to - 1,
            }

            previous_range = Some(*current_range);
        }

        if let Some(previous_range) = previous_range {
            if to > previous_range.to {
                #[cfg(test) ]
                println!("Adding {} to make up from end of range {:?} to stop {}",
                         to - previous_range.to,
                         previous_range,
                         to);
                count = count + to - previous_range.to;
            }
        }

        return count;
    }
}


#[test]
fn test_range_from_string() {
    let src1 = "56-42";
    let range1 = Range::from_str(src1);
    assert_eq!(range1, Ok(Range { from: 56, to: 42 }));

    let src2 = "5";
    assert_eq!(Range::from_str(src2), Err("Not a valid range"));
}

#[test]
fn test_rangeset_add_to_empty() {
    let mut rangeset = RangeSet::new();
    let range = Range { from: 4, to: 6 };

    rangeset.add_range(range);

    assert_eq!(rangeset.ranges.get(&4), Some(&range));
}

#[test]
fn test_rangeset_add_to_nonempty() {
    let mut rangeset = RangeSet::new();
    let range = Range { from: 4, to: 6 };

    rangeset.add_range(range);

    assert_eq!(rangeset.ranges.get(&4), Some(&range));

    // shouldn't change anything
    let range2 = Range { from: 5, to: 6 };
    rangeset.add_range(range2);
    assert_eq!(rangeset.ranges.len(), 1);
    assert_eq!(rangeset.ranges.get(&4), Some(&range));

    // should add new disjoint range
    let range3 = Range { from: 10, to: 16 };
    rangeset.add_range(range3);
    assert_eq!(rangeset.ranges.len(), 2);
    assert_eq!(rangeset.ranges.get(&4), Some(&range));
    assert_eq!(rangeset.ranges.get(&10), Some(&range3));

    // should extend first range
    let range4 = Range { from: 5, to: 8 };
    rangeset.add_range(range4);
    assert_eq!(rangeset.ranges.len(), 2);
    assert_eq!(rangeset.ranges.get(&4), Some(&Range { from: 4, to: 8 }));
    assert_eq!(rangeset.ranges.get(&10), Some(&range3));

    // should extend second range
    let range5 = Range { from: 16, to: 18 };
    rangeset.add_range(range5);
    assert_eq!(rangeset.ranges.len(), 2);
    assert_eq!(rangeset.ranges.get(&4), Some(&Range { from: 4, to: 8 }));
    assert_eq!(rangeset.ranges.get(&10), Some(&Range { from: 10, to: 18 }));
}

#[test]
fn test_make_rangeset() {
    let ranges = vec![Range { from: 4, to: 6 },
                      Range { from: 5, to: 6 },
                      Range { from: 5, to: 8 },
                      Range { from: 16, to: 18 },
                      Range { from: 12, to: 16 },
                      Range { from: 9, to: 10 }];

    let rangeset = RangeSet::from_ranges(ranges);

    // should be the same result as the previous test
    assert_eq!(rangeset.ranges.len(), 2);
    assert_eq!(rangeset.ranges.get(&4), Some(&Range { from: 4, to: 10 }));
    assert_eq!(rangeset.ranges.get(&12), Some(&Range { from: 12, to: 18 }));
}

#[test]
fn test_lowest_gap() {
    let ranges = vec![Range { from: 4, to: 6 },
                      Range { from: 5, to: 6 },
                      Range { from: 5, to: 8 },
                      Range { from: 16, to: 18 },
                      Range { from: 10, to: 16 }];

    let rangeset = RangeSet::from_ranges(ranges);

    assert_eq!(rangeset.lowest_gap(), 0);
}

#[test]
fn test_lowest_gap_2() {
    let ranges = vec![Range { from: 0, to: 6 },
                      Range { from: 5, to: 6 },
                      Range { from: 5, to: 8 },
                      Range { from: 16, to: 18 },
                      Range { from: 10, to: 16 }];

    let rangeset = RangeSet::from_ranges(ranges);

    assert_eq!(rangeset.lowest_gap(), 9);
}

#[test]
fn test_not_included() {
    let ranges = vec![Range { from: 0, to: 6 },
                      Range { from: 5, to: 6 },
                      Range { from: 5, to: 8 },
                      Range { from: 16, to: 18 },
                      Range { from: 10, to: 16 }];

    let rangeset = RangeSet::from_ranges(ranges);

    assert_eq!(rangeset.how_many_not_included(0, 20), 3);
}
