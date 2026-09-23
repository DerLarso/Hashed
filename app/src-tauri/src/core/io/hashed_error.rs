use std::fmt;
use std::io;

#[derive(Debug)]
pub enum HashedError {
    FileNotFound(String),
    InvalidEnding(String),
    HashMissmatch {expected: String, actual: String},
    UnsupportedVersion(usize),
    FileIsCorrupted(String),
    Io(io::Error),
    JsonError(serde_json::Error),
    ZipError(zip::result::ZipError),
}

impl fmt::Display for HashedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashedError::FileNotFound(_e) => write!(f,""),
            HashedError::HashMissmatch { expected: _e, actual: _b } => write!(f,""),
            HashedError::Io(_e) => write!(f, ""),
            HashedError::UnsupportedVersion(_e) => write!(f, ""),
            HashedError::InvalidEnding(_e) => write!(f, ""),
            HashedError::FileIsCorrupted(_e) => write!(f, ""),
            HashedError::JsonError(_e) => write! (f, ""),
            HashedError::ZipError(_e) => write!(f, ""),
        }
    }
}

impl std::error::Error for HashedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HashedError::Io(e) => Some(e),
            HashedError::ZipError(e) => Some(e),
            HashedError::JsonError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for HashedError {
    fn from(err: io::Error) -> Self {
        HashedError::Io(err)
    }
}

impl From<serde_json::Error> for HashedError {
    fn from(err: serde_json::Error) -> Self {
        HashedError::JsonError(err)
    }
}

impl From<zip::result::ZipError> for HashedError {
    fn from(err: zip::result::ZipError) -> Self {
        HashedError::ZipError(err)
    }
}

