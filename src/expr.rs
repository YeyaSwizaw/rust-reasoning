use std::fmt;
use std::fmt::{Display, Formatter};
use std::collections::BTreeSet;
use std::iter::FromIterator;

use error::ParseError;

#[derive(Eq, PartialOrd, Ord, Clone)]
pub enum Expr {
    Atom(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    IfThen(Box<Expr>, Box<Expr>),
    IFF(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn reduce(&self) -> Expr {
        match self {
            &Expr::Atom(_) => self.clone(),

            &Expr::Not(box ref expr) => match expr {
                // DNE
                &Expr::Not(box ref e) => e.clone().reduce(),

                // De Morgan's Law
                &Expr::And(box ref e1, box ref e2) => Expr::Or(box Expr::Not(box e1.clone()), box Expr::Not(box e2.clone())).reduce(),
                &Expr::Or(box ref e1, box ref e2) => Expr::And(box Expr::Not(box e1.clone()), box Expr::Not(box e2.clone())).reduce(),

                _ => Expr::Not(box expr.clone().reduce())
            },

            &Expr::Or(box ref e1, box ref e2) => match (e1, e2) {
                // Distributivity Conjunction
                (&Expr::And(box ref ae1, box ref ae2), oe)
                    | (oe, &Expr::And(box ref ae1, box ref ae2)) => Expr::And(box Expr::Or(box oe.clone(), box ae1.clone()), box Expr::Or(box oe.clone(), box ae2.clone())).reduce(),

                _ => Expr::Or(box e1.clone().reduce(), box e2.clone().reduce()),   
            },

            &Expr::And(box ref e1, box ref e2) => Expr::And(box e1.clone().reduce(), box e2.clone().reduce()),

            &Expr::IfThen(box ref e1, box ref e2) => Expr::Or(box Expr::Not(box e1.clone()), box e2.clone()).reduce(),

            &Expr::IFF(box ref e1, box ref e2) => Expr::And(box Expr::IfThen(box e1.clone(), box e2.clone()), box Expr::IfThen(box e2.clone(), box e1.clone())),
        }
    }

    pub fn cnf_and(&self) -> Result<BTreeSet<BTreeSet<Expr>>, ParseError> {
        match self {
            &Expr::Atom(_) | &Expr::Not(box Expr::Atom(_)) => {
                let mut outer = BTreeSet::new();
                let mut inner = BTreeSet::new();
                inner.insert(self.clone());
                outer.insert(inner);
                Ok(outer)
            },

            &Expr::And(box ref e1, box ref e2) => match e1.cnf_and() {
                Ok(ref mut v1) => e2.cnf_and().map(|ref mut v2| BTreeSet::from_iter(v1.union(v2).cloned())),
                Err(e) => Err(e)
            },

            &Expr::Or(_, _) => match self.cnf_or() {
                Ok(v) => {
                    let mut set = BTreeSet::new();
                    set.insert(v);
                    Ok(set)
                },

                Err(e) => Err(e)
            },

            _ => Err(ParseError::CNFError)
        }
    }

    pub fn cnf_or(&self) -> Result<BTreeSet<Expr>, ParseError> {
        match self {
            &Expr::Atom(_) | &Expr::Not(box Expr::Atom(_)) => {
                let mut set = BTreeSet::new();
                set.insert(self.clone());
                Ok(set)
            },

            &Expr::Or(box ref e1, box ref e2) => match e1.cnf_or() {
                Ok(ref mut v1) => e2.cnf_or().map(|ref v2| BTreeSet::from_iter(v1.union(v2).cloned())),
                Err(e) => Err(e)
            },

            _ => Err(ParseError::CNFError)
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        match (self, other) {
            (&Expr::Atom(ref cl), &Expr::Atom(ref cr)) => cl == cr,
            (&Expr::Not(box ref el), &Expr::Not(box ref er)) => el == er,
            (&Expr::IfThen(box ref el1, box ref el2), &Expr::IfThen(box ref er1, box ref er2)) => (el1 == er1) && (el2 == er2),
            (&Expr::And(box ref el1, box ref el2), &Expr::And(box ref er1, box ref er2)) => ((el1 == er1) && (el2 == er2)) || ((el1 == er2) && (el2 == er1)),
            (&Expr::Or(box ref el1, box ref el2), &Expr::Or(box ref er1, box ref er2)) => ((el1 == er1) && (el2 == er2)) || ((el1 == er2) && (el2 == er1)),
            (&Expr::IFF(box ref el1, box ref el2), &Expr::IFF(box ref er1, box ref er2)) => ((el1 == er1) && (el2 == er2)) || ((el1 == er2) && (el2 == er1)),

            _ => false
        }
    }
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            &Expr::Atom(ref str) => write!(fmt, "{}", str),
            &Expr::Not(ref e) => write!(fmt, "¬{}", e),
            &Expr::And(ref e1, ref e2) => write!(fmt, "({} ∧ {})", e1, e2),
            &Expr::Or(ref e1, ref e2) => write!(fmt, "({} ∨ {})", e1, e2),
            &Expr::IfThen(ref e1, ref e2) => write!(fmt, "({} → {})", e1, e2),
            &Expr::IFF(ref e1, ref e2) => write!(fmt, "({} ↔ {})", e1, e2),
        }
    }
}

impl Display for Vec<Expr> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut first = true;
        self.iter().fold(Ok(()), |acc, expr| acc.and_then(|_| {
            if first {
                first = false;
                write!(fmt, "{}", expr)
            } else {
                write!(fmt, ", {}", expr)
            }
        }))
    }
}

impl Display for BTreeSet<Expr> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut first = true;
        self.iter().fold(Ok(()), |acc, expr| acc.and_then(|_| {
            if first {
                first = false;
                write!(fmt, "{}", expr)
            } else {
                write!(fmt, ", {}", expr)
            }
        }))
    }
}

impl Display for BTreeSet<BTreeSet<Expr>> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut first = true;

        write!(fmt, "{{").and_then(|_| self.iter().fold(Ok(()), |acc, set| {
            if first {
                first = false;
                write!(fmt, "{{{}}}", set)
            } else {
                write!(fmt, ", {{{}}}", set)
            }
        }))
        .and_then(|_| write!(fmt, "}}"))
    }
}
