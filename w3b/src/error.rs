use std::{error::Error as StdError, fmt};

pub enum Error {
    Codec(serde_json::Error),
    Provider(Box<dyn StdError>),
}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Codec(error) => error.fmt(f),
            Error::Provider(error) => error.fmt(f),
        }
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl StdError for Error {}

impl From<serde_json::Error> for Error {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        Error::Codec(error)
    }
}

impl From<reqwest::Error> for Error {
    #[inline]
    fn from(error: reqwest::Error) -> Self {
        Error::Provider(Box::new(error))
    }
}
