use crate::day::{Day, DayResult};

pub struct Day1;

impl Day1 {
    pub fn new() -> Day1 {
        Day1
    }
}

impl Day for Day1 {
    fn run(
        &mut self,
    ) -> std::result::Result<DayResult, std::boxed::Box<(dyn std::error::Error + 'static)>> {
        return Ok(DayResult::default());
    }
}
