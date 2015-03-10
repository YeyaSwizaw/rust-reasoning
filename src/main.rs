#![feature(box_syntax)]

pub mod expr;
pub mod token;

use expr::Expr;
use token::Token;

fn main() {
    let t = vec![Token::Not, Token::Atom("Q".to_string()), Token::IfThen, Token::LParen, Token::Atom("A".to_string()), Token::And, Token::Atom("B".to_string()), Token::RParen];

    let e = Expr::IfThen(box Expr::Not(box Expr::Atom("Q".to_string())), box Expr::And(box Expr::Atom("A".to_string()), box Expr::Atom("B".to_string())));

    println!("{}", t);
    println!("{}", e);
}
