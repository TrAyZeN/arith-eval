use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub index: usize,
}

impl Error {
    pub fn new(kind: ErrorKind, index: usize) -> Error {
        Error {
            kind,
            index,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidCharacter(c) => write!(f, "Found an invalid character: '{}'", c),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidCharacter(char),
}

