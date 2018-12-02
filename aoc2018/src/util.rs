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
