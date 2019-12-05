use crate::day::Day;
use itertools::unfold;
use std::str::FromStr;

pub struct Day1 {
    input: Vec<i32>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            input: parse_input(INPUT).collect(),
        }
    }
}

impl Day for Day1 {
    fn part1(&mut self) -> Result<String, String> {
        let result: i32 = self.input.iter().cloned().map(fuel).sum();
        Ok(format!("Total fuel: {}", result))
    }

    fn part2(&mut self) -> Result<String, String> {
        let result: i32 = self.input.iter().cloned().map(fuel_recursive).sum();
        Ok(format!("Total fuel: {}", result))
    }
}

fn fuel(mass: i32) -> i32 {
    i32::max(0, ((mass as f64 / 3.0).floor() as i32) - 2)
}

fn fuel_recursive(mass: i32) -> i32 {
    unfold(mass, |state| {
        if *state > 0 {
            let new_state = fuel(*state);
            *state = new_state;
            Some(new_state)
        } else {
            None
        }
    })
    .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .map(|line| i32::from_str(line).expect("Should have been an i32. Your input sucks."))
}

#[test]
fn fuel_examples() {
    assert_eq!(fuel(12), 2, "first example");
    assert_eq!(fuel(14), 2, "second example");
    assert_eq!(fuel(1969), 654, "third example");
    assert_eq!(fuel(100756), 33583, "fourth example");
}

const INPUT: &'static str = "78390
73325
52095
126992
106546
81891
69484
131138
95103
53296
115594
79485
130202
95238
99332
136331
124321
127271
108047
69186
90597
96001
138773
55284
127936
110780
89949
85360
55470
110124
101201
139745
148170
149108
79579
139733
52014
125910
77323
145751
52161
105606
132240
69907
144129
116958
60818
144964
111789
85657
115509
84509
50702
69012
110376
134213
127319
92966
58422
144491
128198
84367
94269
147895
105494
88369
117882
146239
50704
62591
149258
63118
145393
122997
136534
96402
121057
59561
86916
75873
68670
147465
62902
145858
137810
108108
97314
118001
54699
56603
66821
80744
124514
143113
132581
79376
105728
115337
111028
52209
";
