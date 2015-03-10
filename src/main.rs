#![feature(box_syntax)]

pub mod expr;
pub mod token;
pub mod error;

use expr::Expr;
use token::Token;
use error::ParseError;

use std::env::args;

fn tokenize_string(string: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();

    let mut chars = string.chars();

    loop {
        match chars.next() {
            Some(ch) => match ch {
                c @ 'A'...'Z' => tokens.push(Token::Atom(c.to_string())),
                '-' | '~' | '¬' => tokens.push(Token::Not),
                '&' | '∧' => tokens.push(Token::And),
                '|' | '∨' => tokens.push(Token::Or),
                '→' => tokens.push(Token::IfThen),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),

                '=' => match chars.next() {
                    Some('>') => {
                        tokens.push(Token::IfThen);
                    },

                    Some(c) => return Err(ParseError::TokenizeError(c.to_string())),
                    None => return Err(ParseError::TokenizeError(ch.to_string()))
                },

                ' ' | '\t' => (),
                c => return Err(ParseError::TokenizeError(c.to_string()))
            },

            None => break
        }
    }

    Ok(tokens)
}

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
                        return Err(ParseError::NotError(token.clone()))
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
                            return Err(ParseError::NotError(token.clone()))
                        },

                        Some(&Token::And) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::And(box expr2, box expr1))
                        } else {
                            return Err(ParseError::NotError(token.clone()))
                        },

                        Some(&Token::Or) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::Or(box expr2, box expr1))
                        } else {
                            return Err(ParseError::NotError(token.clone()))
                        },

                        Some(&Token::IfThen) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::IfThen(box expr2, box expr1))
                        } else {
                            return Err(ParseError::NotError(token.clone()))
                        },

                        Some(&Token::Atom(_)) | Some(&Token::RParen) | None => return Err(ParseError::NotError(token.clone()))
                    }
                }
            }
        }
    }

    loop {
        match token_stack.pop() {
            Some(t @ &Token::Atom(_)) | Some(t @ &Token::LParen) | Some(t @ &Token::RParen) => return Err(ParseError::NotError(t.clone())),

            Some(t @ &Token::Not) => if let Some(expr) = expr_stack.pop() {
                expr_stack.push(Expr::Not(box expr))
            } else {
                return Err(ParseError::NotError(t.clone()))
            },

            Some(t @ &Token::And) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::And(box expr2, box expr1))
            } else {
                return Err(ParseError::NotError(t.clone()))
            },

            Some(t @ &Token::Or) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::Or(box expr2, box expr1))
            } else {
                return Err(ParseError::NotError(t.clone()))
            },

            Some(t @ &Token::IfThen) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::IfThen(box expr2, box expr1))
            } else {
                return Err(ParseError::NotError(t.clone()))
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

fn tokenize_step(string: String) -> Result<Vec<Token>, ParseError> {
    println!("Input: {}", string);
    tokenize_string(&string)
}

fn parse_step(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    println!("Tokens: {}", tokens);
    parse_tokens(&tokens)
}

fn main() {
    let mut args = args();
    
    let parse_result = match (args.next(), args.next()) {
        (_, Some(s)) => Ok(s.to_string()),
        (_, None) => Err(ParseError::NoInput)
    }
    .and_then(tokenize_step)
    .and_then(parse_step);

    match parse_result {
        Ok(ref expr) => println!("Parsed: {}", expr),
        Err(ref err) => println!("{}", err)
    }
}
