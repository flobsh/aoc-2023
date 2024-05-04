use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GenericError,
    IoError(std::io::Error),
    ParseError(String),
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericError => write!(f, "Generic error"),
            Self::IoError(err) => err.fmt(f),
            Self::ParseError(err) => write!(f, "Parse error: {err}"),
            Self::ParseIntError(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GenericError => None,
            Self::IoError(err) => Some(err),
            Self::ParseError(_) => None,
            Self::ParseIntError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
