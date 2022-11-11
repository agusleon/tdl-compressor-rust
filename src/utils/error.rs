use std::{
    fmt,
};


#[derive(Debug)]
pub enum CompressionError {
    IOError(std::io::Error),
    UTFError(std::str::Utf8Error),
    ParseIntegerError(std::num::ParseIntError),
    FullNode
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::IOError(err) => write!(f, "{}", err),
            CompressionError::UTFError(err) => write!(f, "{}", err),
            CompressionError::ParseIntegerError(err) => write!(f, "{}", err),
            CompressionError::FullNode => write!(f, "Node has no empty children")
        }
    }
}


impl From<std::io::Error> for CompressionError {
    fn from(err: std::io::Error) -> Self {
        CompressionError::IOError(err)
    }
}

impl From<std::str::Utf8Error> for CompressionError {
    fn from(err: std::str::Utf8Error) -> Self {
        CompressionError::UTFError(err)
    }
}

impl From<std::num::ParseIntError> for CompressionError {
    fn from(err: std::num::ParseIntError) -> Self {
        CompressionError::ParseIntegerError(err)
    }
}

impl std::error::Error for CompressionError {}
