use std::fmt;

/// The error type for raur.
#[derive(Debug)]
pub enum Error {
    /// The aur returned an error via json.
    Aur(String),
    /// Reqwest returned an error.
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Aur(ref s) => write!(fmt, "{}", s),
            Error::Reqwest(ref e) => write!(fmt, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Aur(_) => None,
            Error::Reqwest(ref e) => e.source(),
        }
    }
}
