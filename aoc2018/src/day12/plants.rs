use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Plants {
    pots: Vec<bool>,
}

impl FromStr for Plants {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut pots = vec![];

        for c in input.chars() {
            if c == '#' {
                pots.push(true);
            } else if c == '.' {
                pots.push(false);
            }
        }

        Ok(Plants { pots })
    }
}

#[test]
fn initial_state_parses() {
    let input = "#..#.#..##......###...###";
    let plants = Plants::from_str(input);

    assert_eq!(
        plants,
        Ok(Plants {
            pots: vec![
                true, false, false, true, false, true, false, false, true, true, false, false,
                false, false, false, false, true, true, true, false, false, false, true, true,
                true
            ]
        })
    );
}
