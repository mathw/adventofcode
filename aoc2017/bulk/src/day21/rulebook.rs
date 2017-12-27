use super::rule::Rule;
use super::grid::Grid;
use super::grid::merge_grids;
use std::str::FromStr;

#[derive(Debug)]
pub struct Rulebook {
    rules: Vec<Rule>,
}

impl Rulebook {
    pub fn new(rules: Vec<Rule>) -> Rulebook {
        Rulebook { rules: rules }
    }

    pub fn apply_to(&self, input: &Grid) -> Option<Grid> {
        let squares = if input.lines.len() % 2 == 0 {
            input.as_twos()
        } else if input.lines.len() % 3 == 0 {
            input.as_threes()
        } else {
            None
        };

        let transformed = squares.map(|s| {
            s.iter()
                .map(|line| {
                    line.iter()
                        .map(|square| {
                            self.apply_single(square)
                                .expect("Application should work if the rulebook is right")
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        });

        transformed.and_then(|t| merge_grids(&t))
    }

    fn apply_single(&self, grid: &Grid) -> Option<Grid> {
        #[cfg(test)]
        println!(
            "Applying {} rules to a single grid segment {}",
            self.rules.len(),
            grid
        );

        for rule in self.rules.iter().cloned() {
            #[cfg(test)]
            println!("Checking rule {}", rule);

            let r = rule.apply_to(grid);
            if r.is_some() {
                return r;
            }
        }

        None
    }
}

impl FromStr for Rulebook {
    type Err = ();

    fn from_str(s: &str) -> Result<Rulebook, ()> {
        s.lines()
            .map(|l| Rule::from_str(l))
            .collect::<Result<Vec<_>, _>>()
            .map(|rules| Rulebook::new(rules))
    }
}


#[cfg(test)]
mod tests {
    use super::super::grid::Grid;
    use std::str::FromStr;
    use super::*;

    fn get_sample_rulebook() -> Rulebook {
        let rulebook_input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        Rulebook::from_str(rulebook_input).expect("Rulebook should parse")
    }

    #[test]
    fn apply_sample_step_1() {
        let input = ".#./..#/###";

        let start_grid = Grid::from_str(input).expect("Starting grid should parse");
        let rulebook = get_sample_rulebook();

        let result = rulebook.apply_to(&start_grid);

        assert_eq!(
            result.unwrap().lines(),
            &vec![
                vec![true, false, false, true],
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, false, false, true],
            ]
        );
    }

    #[test]
    fn apply_sample_step_2() {
        let rulebook = get_sample_rulebook();
        let start_grid = Grid::from_str("#..#/..../..../#..#").expect("Starting grid should parse");
        let result = rulebook.apply_to(&start_grid);
        assert_eq!(
            format!("{}", result.unwrap()),
            "##.##./#..#../....../##.##./#..#../......"
        );
    }
}
