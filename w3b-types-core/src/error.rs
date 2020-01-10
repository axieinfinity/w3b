use std::{error::Error, fmt};

#[derive(PartialEq)]
pub enum TypeError {
    SliceTooLong { len: usize, max: usize },
}

impl fmt::Debug for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::SliceTooLong { len, max } => write!(
                f,
                "slice length too long at {}, expected at most {}",
                len, max
            ),
        }
    }
}

impl fmt::Display for TypeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl Error for TypeError {}
