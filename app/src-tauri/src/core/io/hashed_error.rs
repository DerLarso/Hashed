use std::fmt;
use std::io;
#[derive(Debug)]
pub enum HashedError {
    FileNotFound(String),
    HashMissmatch {expected: String, actual: String},
    UnsupportedVersion(usize),
    Io(io::Error),
}

impl fmt::Display for HashedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashedError::FileNotFound(_e) => write!(f,""),
            HashedError::HashMissmatch { expected: _e, actual: _b } => write!(f,""),
            HashedError::Io(_e) => write!(f, ""),
            HashedError::UnsupportedVersion(_e) => write!(f, ""),
        }
    }
}

impl std::error::Error for HashedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HashedError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for HashedError {
    fn from(err: io::Error) -> Self {
        HashedError::Io(err)
    }
}