use std::fmt::{Display, Formatter, Result};
use token::Token;

pub enum ParseError {
    NoInput,
    TokenizeError(String),
    NotError(Token),
    PopError,
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            &ParseError::NoInput => write!(fmt, "Please provide a formula string"),
            &ParseError::TokenizeError(ref s)  => write!(fmt, "Unexpected character(s): {}", s),
            &ParseError::NotError(ref token) => write!(fmt, "Error popping 'Not' for: {}", token),
            &ParseError::PopError => write!(fmt, "Expression stack not emptied")
        }
    }
}
