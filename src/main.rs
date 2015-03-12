#![feature(box_syntax)]
#![feature(box_patterns)]

pub mod expr;
pub mod token;
pub mod error;

use expr::Expr;
use token::Token;
use error::ParseError;

use std::env::args;
use std::collections::BTreeSet;
use std::iter::FromIterator;

fn tokenize_string(string: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();

    let mut chars = string.chars();

    loop {
        match chars.next() {
            Some(ch) => match ch {
                c @ 'A'...'Z' => tokens.push(Token::Atom(c.to_string())),
                '-' | '~' | '¬' => tokens.push(Token::Not),
                '&' | '∧' | '^' => tokens.push(Token::And),
                '|' | '∨' | 'v' => tokens.push(Token::Or),
                '→' => tokens.push(Token::IfThen),
                '↔' => tokens.push(Token::IfThen),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),

                '=' => match chars.next() {
                    Some('>') => tokens.push(Token::IfThen),
                    Some(c) => return Err(ParseError::TokenizeError(c.to_string())),
                    None => return Err(ParseError::TokenizeError(ch.to_string()))
                },

                '<' => match (chars.next(), chars.next()) {
                    (Some('='), Some('>')) => tokens.push(Token::IFF),
                    _ => return Err(ParseError::TokenizeError(ch.to_string()))
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

            &Token::And | &Token::Or | &Token::IfThen | &Token::IFF => {
                loop {
                    match token_stack.pop() {
                        Some(&Token::Not) => if let Some(expr) = expr_stack.pop() {
                            expr_stack.push(Expr::Not(box expr));
                        } else {
                            return Err(ParseError::NotError(token.clone()))
                        },

                        Some(t) => {
                            token_stack.push(t);
                            break
                        },

                        None => break
                    }
                }

                token_stack.push(token)
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

                        Some(&Token::IFF) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                            expr_stack.push(Expr::IFF(box expr2, box expr1))
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

            Some(t @ &Token::IFF) => if let (Some(expr1), Some(expr2)) = (expr_stack.pop(), expr_stack.pop()) {
                expr_stack.push(Expr::IFF(box expr2, box expr1))
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

fn input_step() -> Result<String, ParseError> {
    let mut args = args();
    
    match (args.next(), args.next()) {
        (_, Some(s)) => Ok(s.to_string()),
        (_, None) => Err(ParseError::NoInput)
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

fn cnf_step(expr: Expr) -> Result<BTreeSet<BTreeSet<Expr>>, ParseError> {
    println!("Expression: {}", expr);

    let mut oldexpr: Expr;
    let mut newexpr = Expr::Not(box expr);

    println!("Negated: {}", newexpr);

    loop {
        oldexpr = newexpr.clone();
        newexpr = newexpr.reduce();

        if newexpr == oldexpr {
            break
        }
    }

    match newexpr {
        Expr::And(_, _) | Expr::Or(_, _) => newexpr.cnf_and(),
        _ => Err(ParseError::CNFError)
    }
}

fn resolution_step(formula: BTreeSet<BTreeSet<Expr>>) -> Result<Vec<(BTreeSet<Expr>, BTreeSet<Expr>, BTreeSet<Expr>)>, ParseError> {
    println!("Reduced: {}", formula);
    println!("");

    let mut form = formula.clone();
    let mut resolved: BTreeSet<BTreeSet<Expr>> = BTreeSet::new();

    let mut new;
    let mut thing_done;

    let mut steps = Vec::new();

    loop {
        form = BTreeSet::from_iter(form.iter().filter(|part| {
            for atom in part.iter() {
                let not = Expr::Not(box atom.clone()).reduce();
                if part.contains(&not) {
                    return false;
                }
            }

            true
        }).cloned());

        new = BTreeSet::new();
        thing_done = false;

        for part in form.iter() {
            if !resolved.contains(part) {
                for atom in part.iter() {
                    let not = Expr::Not(box atom.clone()).reduce();
                    for part2 in form.iter() {
                        if !resolved.contains(part2) && part2.contains(&not) {
                            resolved.insert(part.clone());
                            resolved.insert(part2.clone());

                            let mut s = BTreeSet::new();
                            s.insert(atom.clone());
                            s.insert(not.clone());

                            let p = BTreeSet::from_iter(part.union(part2).cloned());
                            new = BTreeSet::from_iter(p.difference(&s).cloned());
                            steps.push((part.clone(), part2.clone(), new.clone()));
                            thing_done = true;
                            break
                        }
                    }

                    if !new.is_empty() {
                        break
                    }
                }

                if !new.is_empty() {
                    break
                }
            }
        }

        if !thing_done {
            return Ok(Vec::new())
        }

        
        if new.is_empty() {
            return Ok(steps)
        }

        form.insert(new);
    }
}

fn main() {
    println!("");

    let result = input_step()
        .and_then(tokenize_step)
        .and_then(parse_step)
        .and_then(cnf_step)
        .and_then(resolution_step);

    match result {
        Ok(ref vec) => if vec.is_empty() {
            println!("Unsatisfiable!")
        } else {
            println!("Satisfiable!");
            for &(ref a, ref b, ref c) in vec.iter() {
                println!("{{{}}} {{{}}} → {{{}}}", a, b, c)
            }
        },

        Err(ref err) => println!("{}", err)
    }

    println!("");
}
