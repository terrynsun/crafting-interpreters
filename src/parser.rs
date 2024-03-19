#![allow(dead_code)]

use std::rc::Rc;

use crate::token::{
    Token,
    TokenData::{self, *},
};

// expression     → literal
//                | unary
//                | binary
//                | grouping ;
//
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

// Precedence: (lowest = highest)
//
// Equality (== !=)
// Comparison (> >= < <=)
// Term (- +)
// Factor (/ *)
// Unary (! -)
//
// expression     → equality
// equality       → comparison ( (!= | ==) comparison )*
// comparison     → term (( "<>" etc ) term)*
// term           → factor (( "-" | "+" ) factor)*
// factor         → unary ( ("/" | "*") unary )*
// unary          → ("!" | "-") unary | primary
// primary        → literal | "(" expression ")"

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq(Rc<Expr>, Rc<Expr>),
    Neq(Rc<Expr>, Rc<Expr>),

    Gt(Rc<Expr>, Rc<Expr>),
    GtEq(Rc<Expr>, Rc<Expr>),
    Lt(Rc<Expr>, Rc<Expr>),
    LtEq(Rc<Expr>, Rc<Expr>),

    Add(Rc<Expr>, Rc<Expr>),
    Sub(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Mult(Rc<Expr>, Rc<Expr>),

    Negative(Rc<Expr>),
    Inverse(Rc<Expr>),

    Literal(LiteralExpr),
}

macro_rules! pretty {
    ( $s:literal, $left:expr, $right:expr, $indent:expr) => {
        {
            $left.pretty_recur($indent+4);
            println!("{}{}", " ".repeat($indent), $s);
            $right.pretty_recur($indent+4);
        }
    }
}

impl Expr {
    pub fn pretty(&self) {
        self.pretty_recur(0)
    }

    pub fn pretty_recur(&self, indent: usize) {
        match self {
            Expr::Eq(left, right) => pretty!("==", left, right, indent),
            Expr::Neq(left, right) => pretty!("!=", left, right, indent),

            Expr::Gt(left, right) => pretty!("<", left, right, indent),
            Expr::GtEq(left, right) => pretty!("<", left, right, indent),
            Expr::Lt(left, right) => pretty!("<", left, right, indent),
            Expr::LtEq(left, right) => pretty!("<", left, right, indent),

            Expr::Add(left, right) => pretty!("+", left, right, indent),
            Expr::Sub(left, right) => pretty!("-", left, right, indent),
            Expr::Div(left, right) => pretty!("*", left, right, indent),
            Expr::Mult(left, right) => pretty!("/", left, right, indent),

            Expr::Negative(e) => {
                println!("{}-", " ".repeat(indent));
                e.pretty_recur(indent+4);
            }
            Expr::Inverse(e) => {
                println!("{}!", " ".repeat(indent));
                e.pretty_recur(indent+4);
            }

            Expr::Literal(l) => {
                println!("{}{:?}", " ".repeat(indent), l);
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum LiteralExpr {
    NumberLiteral(f32),
    StringLiteral(String),
    True,
    False,
    Nil,
}

impl std::fmt::Debug for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralExpr::NumberLiteral(n) => write!(f, "{}", n),
            LiteralExpr::StringLiteral(s) => write!(f, "{}", s),
            LiteralExpr::True => write!(f, "true"),
            LiteralExpr::False => write!(f, "false"),
            LiteralExpr::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::token::{Token, TokenData::*};
    use crate::tokens;

    use super::{parse, Expr::*, LiteralExpr};

    #[test]
    fn bang_literal() {
        let tokens = tokens![(Bang, 0), (False, 0)];
        let expected = Inverse(Rc::new(Literal(LiteralExpr::False)));
        assert_eq!(parse(tokens), expected);
    }
}

struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, idx: 0 }
    }

    /* Utilities for interacting with the token array */
    fn is_at_end(&mut self) -> bool {
        self.idx >= self.tokens.len() - 1
    }

    fn next(&mut self) {
        self.idx += 1;
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.idx]
    }

    fn consume(&mut self) -> &Token {
        self.next();
        &self.tokens[self.idx - 1]
    }

    fn parse(&mut self) -> Expr {
        self.equality()
    }

    /* The recursive descent parser itself */
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        if self.is_at_end() {
            return expr;
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::BangEqual => {
                    self.next();
                    let right = self.comparison();
                    expr = Expr::Neq(expr.clone().into(), right.into());
                },
                TokenData::EqualEqual => {
                    self.next();
                    let right = self.comparison();
                    expr = Expr::Eq(expr.clone().into(), right.into());
                },
                _ => break,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        if self.is_at_end() {
            return expr;
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::Greater => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Gt(expr.clone().into(), right.into());
                },
                TokenData::GreaterEqual => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::GtEq(expr.clone().into(), right.into());
                },
                TokenData::Less => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Lt(expr.clone().into(), right.into());
                },
                TokenData::LessEqual => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::LtEq(expr.clone().into(), right.into());
                },
                _ => break,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        if self.is_at_end() {
            return expr;
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::Plus => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Add(expr.clone().into(), right.into());
                },
                TokenData::Minus => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Sub(expr.clone().into(), right.into());
                },
                _ => break,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        if self.is_at_end() {
            return expr;
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::Slash => {
                    self.next();
                    let right = self.unary();
                    expr = Expr::Div(expr.clone().into(), right.into());
                },
                TokenData::Star => {
                    self.next();
                    let right = self.unary();
                    expr = Expr::Mult(expr.clone().into(), right.into());
                },
                _ => break,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let cur = &self.tokens[self.idx];
        match cur.data {
            Bang => {
                self.next();
                Expr::Inverse(Rc::new(self.unary()))
            }
            Minus => {
                self.next();
                Expr::Negative(Rc::new(self.unary()))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let cur = self.peek();

        match &cur.data {
            StringToken(s) => {
                // clone the literal out of the immutable borrow before modifying self
                let ret = Expr::Literal(LiteralExpr::StringLiteral(s.clone()));

                self.next();

                ret
            }
            Number(n) => {
                // copy the literal out of the immutable borrow before modifying self
                let ret = Expr::Literal(LiteralExpr::NumberLiteral(*n));

                self.next();

                ret
            }
            True => {
                self.next();
                Expr::Literal(LiteralExpr::True)
            }
            False => {
                self.next();
                Expr::Literal(LiteralExpr::False)
            }
            Nil => {
                self.next();
                Expr::Literal(LiteralExpr::Nil)
            }
            LeftParen => {
                self.next(); // first move pointer past LeftParen

                let expr = self.equality();

                // consume RightParen too
                let next_token = self.peek();
                if let RightParen = next_token.data {
                    self.next();
                } else {
                    panic!("expected closing parens, got {next_token:?}");
                }
                expr
            }
            t => panic!("unexpected token: {t:?}"),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut p = Parser::new(tokens);
    p.parse()
}
