mod asmillis;

use std::fmt::Display;
use std::time::Instant;

pub trait ErrString {
    type Success;
    fn err_string(self) -> Result<Self::Success, String>;
}

impl<SuccessType, ErrorType> ErrString for Result<SuccessType, ErrorType>
where
    ErrorType: Display,
{
    type Success = SuccessType;

    fn err_string(self) -> Result<Self::Success, String> {
        self.map_err(|e| e.to_string())
    }
}

pub fn timed<W, R>(work: W) -> (R, u64)
where
    W: Fn() -> R,
{
    let timer = Instant::now();
    // awkward syntax to avoid the warning about as_millis being added to the standard library in the future
    (work(), asmillis::AsMillis::as_millis(&timer.elapsed()))
}
