use rustyline::error::ReadlineError;
use std::convert;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum RLoxError {
    IO {
        kind: std::io::ErrorKind,
    },
    Source {
        line: usize,
        location: String,
        message: String,
    },
    Interrupted,
    EOF,
}

impl RLoxError {
    pub fn source(line: usize, location: String, message: String) -> Self {
        RLoxError::Source {
            line,
            location,
            message,
        }
    }
}

impl std::fmt::Display for RLoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RLoxError::IO { kind } => write!(f, "IO error: {:?}", kind),
            RLoxError::Source {
                line,
                location,
                message,
            } => write!(f, "[{}] Error {}: {}", line, location, message),
            RLoxError::Interrupted => write!(f, "Interrupted"),
            RLoxError::EOF => write!(f, "End  of input"),
        }
    }
}

impl std::error::Error for RLoxError {}

impl convert::From<std::io::Error> for RLoxError {
    fn from(io_error: std::io::Error) -> Self {
        RLoxError::IO {
            kind: io_error.kind(),
        }
    }
}

impl convert::From<ReadlineError> for RLoxError {
    fn from(error: ReadlineError) -> Self {
        match error {
            ReadlineError::Interrupted => RLoxError::Interrupted,
            ReadlineError::Eof => RLoxError::EOF,
            _ => RLoxError::IO {
                kind: ErrorKind::Other,
            },
        }
    }
}

pub type RLoxResult<T> = Result<T, RLoxError>;
