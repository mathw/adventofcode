mod asmillis;

use std::fmt::Display;

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

use self::asmillis::AsMillis;
use std::time::Instant;

pub fn timed<W, R>(work: W) -> (R, u64)
where
    W: Fn() -> R,
{
    let timer = Instant::now();
    (work(), timer.elapsed().as_millis())
}
