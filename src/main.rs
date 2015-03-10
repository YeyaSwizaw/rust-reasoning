#![feature(box_syntax)]

pub mod expr;
pub mod token;
pub mod error;

use expr::Expr;
use token::Token;
use error::ParseError;

fn parse_tokens(tokens: &Vec<Token>) -> Result<Expr, ParseError> {
    let mut token_stack = Vec::new();
    let mut expr_stack = Vec::new();

    for token in tokens.iter() {
        match token {
            &Token::Atom(ref s) => expr_stack.push(Expr::Atom(s.clone())),

            &Token::Not | &Token::LParen => token_stack.push(token),

            &Token::And | &Token::Or | &Token::IfThen => {
                match token_stack.pop() {
                    Some(&Token::Not) => if let Some(expr) = expr_stack.pop() {
                        expr_stack.push(Expr::Not(box expr));
                        token_stack.push(token)
                    } else {
                        return Err(ParseError::NotError(token))
                    },

                    Some(t) => {
                        token_stack.push(t);
                        token_stack.push(token)
                    },

                    None => token_stack.push(token)
                }
            }

            &Token::RParen => {
                loop {
                    match token_stack.pop() {
                        Some(&Token::LParen) => break,

                        Some(&Token::Not) => if let Some(expr) = expr_stack.pop() {
                            expr_stack.push(Expr::Not(box expr))
                        } else {
                            return Err(ParseError::NotError(token))
                        },

                        Some(&Token::And) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::And(box expr2, box expr1))
                        } else {
                            return Err(ParseError::NotError(token))
                        },

                        Some(&Token::Or) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::Or(box expr2, box expr1))
                        } else {
                            return Err(ParseError::NotError(token))
                        },

                        Some(&Token::IfThen) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::IfThen(box expr2, box expr1))
                        } else {
                            return Err(ParseError::NotError(token))
                        },

                        Some(&Token::Atom(_)) | Some(&Token::RParen) | None => return Err(ParseError::NotError(token))
                    }
                }
            }
        }
    }

    loop {
        match token_stack.pop() {
            Some(t @ &Token::Atom(_)) | Some(t @ &Token::LParen) | Some(t @ &Token::RParen) => return Err(ParseError::NotError(t)),

            Some(t @ &Token::Not) => if let Some(expr) = expr_stack.pop() {
                expr_stack.push(Expr::Not(box expr))
            } else {
                return Err(ParseError::NotError(t))
            },

            Some(t @ &Token::And) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::And(box expr2, box expr1))
            } else {
                return Err(ParseError::NotError(t))
            },

            Some(t @ &Token::Or) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::Or(box expr2, box expr1))
            } else {
                return Err(ParseError::NotError(t))
            },

            Some(t @ &Token::IfThen) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::IfThen(box expr2, box expr1))
            } else {
                return Err(ParseError::NotError(t))
            },

            None => break
        }
    }

    if let (Some(expr), None) = (expr_stack.pop(), expr_stack.pop()) {
        Ok(expr)
    } else {
        return Err(ParseError::PopError)
    }
}

fn main() {
    let t = vec![Token::LParen, Token::Not, Token::Atom("Q".to_string()), Token::IfThen, Token::LParen, Token::Atom("A".to_string()), Token::And, Token::Atom("B".to_string()), Token::RParen, Token::RParen, Token::Or, Token::Not, Token::Atom("A".to_string())];

    // let e = Expr::IfThen(box Expr::Not(box Expr::Atom("Q".to_string())), box Expr::And(box Expr::Atom("A".to_string()), box Expr::Atom("B".to_string())));

    println!("{}", t);

    match parse_tokens(&t) {
        Ok(ref expr) => println!("{}", expr),
        Err(ref err) => println!("{}", err)
    }
}
