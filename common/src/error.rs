#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    LengthMismatch,
    TimestampOrderMismatch,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::LengthMismatch => write!(f, "Length mismatch"),
            Error::TimestampOrderMismatch => write!(f, "Timestamp order mismatch"),
        }
    }
}

impl std::error::Error for Error {}