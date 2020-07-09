use std::fmt;
use crate::token::Token;

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
        match &self.kind {
            ErrorKind::InvalidCharacter(c) => write!(f, "Found an invalid character: '{}'", c),
            ErrorKind::UnexpectedToken(t1, t2) => write!(f, "Unexpected token '{:?}' near token '{:?}", t2, t1),
            ErrorKind::MissingOperand => write!(f, "Missing operand"),
            ErrorKind::MissingExpression => write!(f, "Missing expresion"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidCharacter(char),
    UnexpectedToken(Token, Token),
    MissingOperand,
    MissingExpression,
}
