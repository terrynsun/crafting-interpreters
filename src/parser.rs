use crate::expr::{BinOp, Expr, Program, Stmt, UnaryOp};
use crate::token::{
    Token,
    TokenData::{self, *},
};

type ParseError = String;

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
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

    fn expect(&mut self, expected: TokenData, err: &'static str) -> Result<(), ParseError> {
        let next_token = self.peek();
        if expected == next_token.data {
            self.next();
            Ok(())
        } else {
            Err(format!("parse error: expected {err}, got {next_token:?}"))
        }
    }

    fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = vec![];

        while !self.is_at_end() {
            let expr = self.statement()?;
            program.push(expr);
        }

        Ok(program)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        let stmt = match self.peek().data {
            Print => {
                self.next();

                let inner = self.parse_expression()?;
                Stmt::PrintStmt(inner)
            }
            _ => {
                let inner = self.parse_expression()?;
                Stmt::Expr(inner)
            }
        };

        // consume semicolon
        self.next();

        Ok(stmt)
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    /* The recursive descent parser itself */
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::BangEqual => {
                    self.next();
                    let right = self.comparison()?;
                    expr = Expr::Binary(BinOp::Neq, expr.clone().into(), right.into());
                }
                TokenData::EqualEqual => {
                    self.next();
                    let right = self.comparison()?;
                    expr = Expr::Binary(BinOp::Eq, expr.clone().into(), right.into());
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::Greater => {
                    self.next();
                    let right = self.factor()?;
                    expr = Expr::Binary(BinOp::Gt, expr.clone().into(), right.into());
                }
                TokenData::GreaterEqual => {
                    self.next();
                    let right = self.factor()?;
                    expr = Expr::Binary(BinOp::GtEq, expr.clone().into(), right.into());
                }
                TokenData::Less => {
                    self.next();
                    let right = self.factor()?;
                    expr = Expr::Binary(BinOp::Lt, expr.clone().into(), right.into());
                }
                TokenData::LessEqual => {
                    self.next();
                    let right = self.factor()?;
                    expr = Expr::Binary(BinOp::LtEq, expr.clone().into(), right.into());
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::Plus => {
                    self.next();
                    let right = self.factor()?;
                    expr = Expr::Binary(BinOp::Add, expr.clone().into(), right.into());
                }
                TokenData::Minus => {
                    self.next();
                    let right = self.factor()?;
                    expr = Expr::Binary(BinOp::Sub, expr.clone().into(), right.into());
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            let next = &self.tokens[self.idx];
            match &next.data {
                TokenData::Slash => {
                    self.next();
                    let right = self.unary()?;
                    expr = Expr::Binary(BinOp::Div, expr.clone().into(), right.into());
                }
                TokenData::Star => {
                    self.next();
                    let right = self.unary()?;
                    expr = Expr::Binary(BinOp::Mult, expr.clone().into(), right.into());
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        let cur = &self.tokens[self.idx];
        match cur.data {
            Minus => {
                self.next();
                let e = self.unary()?;
                Ok(Expr::Unary(UnaryOp::Negative, e.into()))
            }
            Bang => {
                self.next();
                let e = self.unary()?;
                Ok(Expr::Unary(UnaryOp::Inverse, e.into()))
            }
            _ => Ok(self.primary()?),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let cur = self.peek();

        match &cur.data {
            Identifier(s) => {
                // clone the string out of the immutable borrow before modifying self
                let ret = Expr::Identifier(s.clone());

                self.next();

                Ok(ret)
            }
            StringToken(s) => {
                // clone the string out of the immutable borrow before modifying self
                let ret = Expr::StringLiteral(s.clone());

                self.next();

                Ok(ret)
            }
            Number(n) => {
                // copy the literal out of the immutable borrow before modifying self
                let ret = Expr::NumberLiteral(*n);

                self.next();

                Ok(ret)
            }
            True => {
                self.next();
                Ok(Expr::True)
            }
            False => {
                self.next();
                Ok(Expr::False)
            }
            Nil => {
                self.next();
                Ok(Expr::Nil)
            }
            LeftParen => {
                self.next(); // first move pointer past LeftParen

                let expr = self.equality()?;

                // consume RightParen too
                self.expect(TokenData::RightParen, "closing parens")?;

                Ok(expr)
            }
            Eof => Err("parse error: unexpected end of file".to_string()),
            t => Err(format!("parse error: unexpected token: {t:?}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{BinOp, Expr, Stmt, UnaryOp};
    use crate::token::{Token, TokenData};
    use crate::tokens;

    use super::parse;

    macro_rules! parse_expr {
        ( $tokens:expr, $expected:expr ) => {{
            let program = parse($tokens).unwrap();
            assert_eq!(program[0], Stmt::Expr($expected));
        }};
    }

    #[test]
    fn literals() {
        parse_expr!(
            tokens![(TokenData::True, 0), (TokenData::Eof, 0)],
            Expr::True
        );

        parse_expr!(
            tokens![(TokenData::False, 0), (TokenData::Eof, 0)],
            Expr::False
        );

        parse_expr!(tokens![(TokenData::Nil, 0), (TokenData::Eof, 0)], Expr::Nil);

        parse_expr!(
            tokens![(TokenData::Number(1.0), 0), (TokenData::Eof, 0)],
            Expr::NumberLiteral(1.0)
        );

        parse_expr!(
            tokens![
                (TokenData::StringToken("foo".to_string()), 0),
                (TokenData::Eof, 0)
            ],
            Expr::StringLiteral("foo".to_string())
        );
    }

    #[test]
    fn unary() {
        parse_expr!(
            tokens![
                (TokenData::Bang, 0),
                (TokenData::False, 0),
                (TokenData::Eof, 0)
            ],
            Expr::Unary(UnaryOp::Inverse, Expr::False.into())
        );

        parse_expr!(
            tokens![
                (TokenData::Minus, 0),
                (TokenData::False, 0),
                (TokenData::Eof, 0)
            ],
            Expr::Unary(UnaryOp::Negative, Expr::False.into())
        );
    }

    #[test]
    fn cmps() {
        parse_expr!(
            tokens![
                (TokenData::True, 0),
                (TokenData::Greater, 0),
                (TokenData::False, 0),
                (TokenData::Eof, 0)
            ],
            Expr::Binary(BinOp::Gt, Expr::True.into(), Expr::False.into())
        );

        parse_expr!(
            tokens![
                (TokenData::True, 0),
                (TokenData::GreaterEqual, 0),
                (TokenData::False, 0),
                (TokenData::Eof, 0)
            ],
            Expr::Binary(BinOp::GtEq, Expr::True.into(), Expr::False.into())
        );

        parse_expr!(
            tokens![
                (TokenData::True, 0),
                (TokenData::Less, 0),
                (TokenData::False, 0),
                (TokenData::Eof, 0)
            ],
            Expr::Binary(BinOp::Lt, Expr::True.into(), Expr::False.into())
        );

        parse_expr!(
            tokens![
                (TokenData::True, 0),
                (TokenData::LessEqual, 0),
                (TokenData::False, 0),
                (TokenData::Eof, 0)
            ],
            Expr::Binary(BinOp::LtEq, Expr::True.into(), Expr::False.into())
        );

        // left-associativity
        parse_expr!(
            tokens![
                (TokenData::Number(1.0), 0),
                (TokenData::LessEqual, 0),
                (TokenData::Number(2.0), 0),
                (TokenData::GreaterEqual, 0),
                (TokenData::Number(3.0), 0),
                (TokenData::Eof, 0)
            ],
            Expr::Binary(
                BinOp::GtEq,
                Expr::Binary(
                    BinOp::LtEq,
                    Expr::NumberLiteral(1.0).into(),
                    Expr::NumberLiteral(2.0).into()
                ).into(),
                Expr::NumberLiteral(3.0).into()
            )
        );
    }
}
