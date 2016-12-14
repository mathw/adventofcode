mod parser;
mod types;

use self::types::ResearchCentre;

pub fn do_day11(input: &str) {
    let mut research_centre = ResearchCentre::new();

    for line in input.lines() {
        if let Some((floor, contents)) = parser::parse_line(line) {
            research_centre.add_floor_contents(floor, contents);
        }
    }

}
