#![allow(dead_code)]

use std::rc::Rc;

use crate::expr::{Expr, BinOp, UnaryOp};
use crate::token::{
    Token,
    TokenData::{self, *},
};

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut p = Parser::new(tokens);
    p.parse()
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
                    expr = Expr::Binary(BinOp::Neq, expr.clone().into(), right.into());
                }
                TokenData::EqualEqual => {
                    self.next();
                    let right = self.comparison();
                    expr = Expr::Binary(BinOp::Eq, expr.clone().into(), right.into());
                }
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
                    expr = Expr::Binary(BinOp::Gt, expr.clone().into(), right.into());
                },
                TokenData::GreaterEqual => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Binary(BinOp::GtEq, expr.clone().into(), right.into());
                },
                TokenData::Less => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Binary(BinOp::Lt, expr.clone().into(), right.into());
                },
                TokenData::LessEqual => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Binary(BinOp::LtEq, expr.clone().into(), right.into());
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
                    expr = Expr::Binary(BinOp::Add, expr.clone().into(), right.into());
                },
                TokenData::Minus => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::Binary(BinOp::Sub, expr.clone().into(), right.into());
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
                    expr = Expr::Binary(BinOp::Div, expr.clone().into(), right.into());
                },
                TokenData::Star => {
                    self.next();
                    let right = self.unary();
                    expr = Expr::Binary(BinOp::Mult, expr.clone().into(), right.into());
                },
                _ => break,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let cur = &self.tokens[self.idx];
        match cur.data {
            Minus => {
                self.next();
                Expr::Unary(UnaryOp::Negative, Rc::new(self.unary()))
            }
            Bang => {
                self.next();
                Expr::Unary(UnaryOp::Inverse, Rc::new(self.unary()))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let cur = self.peek();

        match &cur.data {
            StringToken(s) => {
                // clone the literal out of the immutable borrow before modifying self
                let ret = Expr::StringLiteral(s.clone());

                self.next();

                ret
            }
            Number(n) => {
                // copy the literal out of the immutable borrow before modifying self
                let ret = Expr::NumberLiteral(*n);

                self.next();

                ret
            }
            True => {
                self.next();
                Expr::True
            }
            False => {
                self.next();
                Expr::False
            }
            Nil => {
                self.next();
                Expr::Nil
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::expr::UnaryOp;
    use crate::token::{Token, TokenData, TokenData::*};
    use crate::tokens;

    use super::{parse, Expr::*};

    #[test]
    fn bang_literal() {
        let tokens = tokens![(Bang, 0), (TokenData::False, 0)];
        let expected = Unary(UnaryOp::Inverse, Rc::new(False));
        assert_eq!(parse(tokens), expected);
    }
}
