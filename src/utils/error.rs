use std::{
    fmt,
    sync::mpsc::SendError,
};

#[derive(Debug)]
pub enum CompressionError {
    IOError(std::io::Error),
    UTFError(std::str::Utf8Error),
    ParseIntegerError(std::num::ParseIntError),
    FullNode,
    PoisonError,
    SenderError(SendError<usize>),
    NotEnoughArguments(String)
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::IOError(err) => write!(f, "{}", err),
            CompressionError::UTFError(err) => write!(f, "{}", err),
            CompressionError::ParseIntegerError(err) => write!(f, "{}", err),
            CompressionError::FullNode => write!(f, "Node has no empty children"),
            CompressionError::PoisonError => write!(f, "The Lock is poisoned."),
            CompressionError::SenderError(err) => write!(f, "{}", err),
            CompressionError::NotEnoughArguments(argument) => write!(f, "You are missing one argument {}", argument),
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

impl From<SendError<usize>> for CompressionError {
    fn from(err: SendError<usize>) -> Self {
        CompressionError::SenderError(err)
    }
}

impl std::error::Error for CompressionError {}
