use std::{
    error::Error,
    fmt::{self, Display},
    io, result,
};

pub type Result<T> = result::Result<T, SpeakError>;

#[derive(Debug, Eq, PartialEq)]
pub enum SpeakError {
    Io(io::ErrorKind),
    Date(String),
}

impl Display for SpeakError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            SpeakError::Io(e) => write!(f, "IO error: {}", e),
            SpeakError::Date(s) => write!(f, "Can't format '{}' as a Date", s),
        }
    }
}

impl Error for SpeakError {}

impl From<io::Error> for SpeakError {
    fn from(e: io::Error) -> Self {
        SpeakError::Io(e.kind())
    }
}
