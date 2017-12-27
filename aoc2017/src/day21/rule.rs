use std::fmt::Formatter;
use super::grid::Grid;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    pattern: Grid,
    output: Grid,
}

impl Rule {
    /// Attempt to apply this rule to the given grid.
    /// Returns `None` if this rule cannot match the input grid.
    pub fn apply_to(&self, input: &Grid) -> Option<Grid> {
        if self.pattern.is_match(input) {
            Some(self.output.clone())
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Rule, ()> {
        let mut segments = s.split(" => ");
        let opt_input_string = segments.next();
        let opt_output_string = segments.next();

        match (opt_input_string, opt_output_string) {
            (Some(input_string), Some(output_string)) => {
                let input = Grid::from_str(input_string)?;
                let output = Grid::from_str(output_string)?;
                Ok(Rule {
                    pattern: input,
                    output: output,
                })
            }
            _ => Err(()),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{} => {}", self.pattern, self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::super::grid::*;
    use super::*;

    #[test]
    fn can_parse_patterns() {
        let input1 = "../.# => ##./#../...";
        let input2 = ".#./..#/### => #..#/..../..../#..#";

        let rule1 = Rule::from_str(input1);
        let rule2 = Rule::from_str(input2);

        assert_eq!(
            rule1,
            Ok(Rule {
                pattern: Grid {
                    lines: vec![vec![false, false], vec![false, true]],
                },
                output: Grid {
                    lines: vec![
                        vec![true, true, false],
                        vec![true, false, false],
                        vec![false, false, false],
                    ],
                },
            })
        );

        assert_eq!(
            rule2,
            Ok(Rule {
                pattern: Grid {
                    lines: vec![
                        vec![false, true, false],
                        vec![false, false, true],
                        vec![true, true, true],
                    ],
                },
                output: Grid {
                    lines: vec![
                        vec![true, false, false, true],
                        vec![false, false, false, false],
                        vec![false, false, false, false],
                        vec![true, false, false, true],
                    ],
                },
            })
        );
    }

    #[test]
    fn can_apply() {
        let rule = Rule::from_str(".#./..#/### => #..#/..../..../#..#").unwrap();
        let input = Grid::from_str(".#./..#/###").unwrap();
        let expected = Grid::from_str("#..#/..../..../#..#").unwrap();
        let result = rule.apply_to(&input);
        assert_eq!(result, Some(expected));
    }
}
