use thiserror::Error;

#[derive(Error, Debug)]
pub enum DayError {
    #[error("No solution found")]
    NoSolutionFoundError,
    #[error("Could not parse input")]
    InputParseError(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
