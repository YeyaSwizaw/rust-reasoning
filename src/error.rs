use std::fmt::{Display, Formatter, Result};
use token::Token;

pub enum ParseError<'a> {
    NotError(&'a Token),
    PopError,

}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            &ParseError::NotError(ref token) => write!(fmt, "Error popping 'Not' for: {}", token),
            &ParseError::PopError => write!(fmt, "Expression stack not emptied")
        }
    }
}
