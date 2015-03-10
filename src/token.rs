use std::fmt::{Display, Formatter, Result};

pub enum Token {
    Atom(String),
    Not,
    And,
    Or,
    IfThen,
    LParen,
    RParen,
}

impl Display for Token {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            &Token::Atom(ref str) => write!(fmt, "Atom({})", str),
            &Token::Not => write!(fmt, "Not"),
            &Token::And => write!(fmt, "And"),
            &Token::Or => write!(fmt, "Or"),
            &Token::IfThen => write!(fmt, "IfThen"),
            &Token::LParen => write!(fmt, "LParen"),
            &Token::RParen => write!(fmt, "RParen"),
        }
    }
}

impl Display for Vec<Token> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let mut first = true;
        self.iter().fold(Ok(()), |acc, token| acc.and_then(|_| {
            if first {
                first = false;
                write!(fmt, "{}", token)
            } else {
                write!(fmt, ", {}", token)
            }
        }))
    }
}










