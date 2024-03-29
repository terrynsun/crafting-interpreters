use crate::error::{Error, ErrorState};
use crate::expr::{BinOp, Expr, Program, Stmt, UnaryOp};
use crate::token::{
    Token,
    TokenData::{self, *},
};

pub fn parse(tokens: Vec<Token>) -> Result<Program, ErrorState> {
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

    fn expect(&mut self, expected: TokenData, err: &'static str) -> Result<(), Error> {
        let next_token = self.peek();
        if expected == next_token.data {
            self.next();
            Ok(())
        } else {
            Err(Error::parse_error(
                format!("expected {err}, got {next_token:?}"),
                next_token.line,
            ))
        }
    }

    /* The recursive descent parser itself */

    // Entrypoint for a full program
    //
    // Error handling: right now, self.statement() can only return a single error. But program can
    // return a bunch of errors. When we hit an error, try to recover by fast-forwarding until we
    // find a semicolon. Then try to parse another statement.
    fn parse(&mut self) -> Result<Program, ErrorState> {
        let mut program = vec![];
        let mut err_state = ErrorState::new_parser_state();

        while !self.is_at_end() {
            match self.statement() {
                Ok(expr) => program.push(expr),
                Err(e) => {
                    err_state.add(e);
                    loop {
                        let next = self.peek();
                        match next.data {
                            Semicolon => {
                                // End of statement. Break out of error recovery and try to parse
                                // next statement.
                                self.next();
                                break;
                            }
                            Eof => {
                                // End of file.
                                break;
                            }
                            _ => {
                                println!("err @ {:?} -- incrementing", next);
                                // Keep skipping forward.
                                self.next();
                            }
                        }
                    }
                }
            }
        }

        if err_state.is_ok() {
            Ok(program)
        } else {
            Err(err_state)
        }
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
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

        self.expect(TokenData::Semicolon, "semicolon")?;

        Ok(stmt)
    }

    fn parse_expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            match &self.peek().data {
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

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            match &self.peek().data {
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

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            match &self.peek().data {
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

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            match &self.peek().data {
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

    fn unary(&mut self) -> Result<Expr, Error> {
        match &self.peek().data {
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

    fn primary(&mut self) -> Result<Expr, Error> {
        let next = self.peek();
        match &next.data {
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

                self.expect(TokenData::RightParen, "closing parens")?;

                Ok(expr)
            }
            Eof => Err(Error::parse_error(
                "unexpected end of file".to_string(),
                next.line,
            )),
            t => Err(Error::parse_error(
                format!("unexpected token: {t:?}"),
                next.line,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{BinOp, Expr, Stmt, UnaryOp};
    use crate::token::{Token, TokenData};
    use crate::tokens;

    use super::parse;

    macro_rules! assert_expr_parses {
        ( $tokens:expr, $expected:expr ) => {{
            let mut v = $tokens.clone();
            // append a semicolon to create a valid program
            v.push(Token::new(TokenData::Semicolon, 0));
            v.push(Token::new(TokenData::Eof, 0));

            let program = parse(v).unwrap();
            assert_eq!(program[0], Stmt::Expr($expected));
        }};
    }

    #[test]
    fn literals() {
        assert_expr_parses!(tokens![TokenData::True], Expr::True);

        assert_expr_parses!(tokens![TokenData::False], Expr::False);

        assert_expr_parses!(tokens![TokenData::Nil], Expr::Nil);

        assert_expr_parses!(tokens![TokenData::Number(1.0)], Expr::NumberLiteral(1.0));

        assert_expr_parses!(
            tokens![TokenData::StringToken("foo".to_string())],
            Expr::StringLiteral("foo".to_string())
        );
    }

    #[test]
    fn unary() {
        assert_expr_parses!(
            tokens![TokenData::Bang, TokenData::False],
            Expr::Unary(UnaryOp::Inverse, Expr::False.into())
        );

        assert_expr_parses!(
            tokens![TokenData::Minus, TokenData::False],
            Expr::Unary(UnaryOp::Negative, Expr::False.into())
        );
    }

    #[test]
    fn cmps() {
        assert_expr_parses!(
            tokens![TokenData::True, TokenData::Greater, TokenData::False,],
            Expr::Binary(BinOp::Gt, Expr::True.into(), Expr::False.into())
        );

        assert_expr_parses!(
            tokens![TokenData::True, TokenData::GreaterEqual, TokenData::False,],
            Expr::Binary(BinOp::GtEq, Expr::True.into(), Expr::False.into())
        );

        assert_expr_parses!(
            tokens![TokenData::True, TokenData::Less, TokenData::False,],
            Expr::Binary(BinOp::Lt, Expr::True.into(), Expr::False.into())
        );

        assert_expr_parses!(
            tokens![TokenData::True, TokenData::LessEqual, TokenData::False,],
            Expr::Binary(BinOp::LtEq, Expr::True.into(), Expr::False.into())
        );

        // left-associativity
        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::LessEqual,
                TokenData::Number(2.0),
                TokenData::GreaterEqual,
                TokenData::Number(3.0),
            ],
            Expr::Binary(
                BinOp::GtEq,
                Expr::Binary(
                    BinOp::LtEq,
                    Expr::NumberLiteral(1.0).into(),
                    Expr::NumberLiteral(2.0).into(),
                )
                .into(),
                Expr::NumberLiteral(3.0).into()
            )
        );
    }

    #[test]
    fn math() {
        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Plus,
                TokenData::Number(2.0),
            ],
            Expr::Binary(
                BinOp::Add,
                Expr::NumberLiteral(1.0).into(),
                Expr::NumberLiteral(2.0).into(),
            )
        );

        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Minus,
                TokenData::Number(2.0),
            ],
            Expr::Binary(
                BinOp::Sub,
                Expr::NumberLiteral(1.0).into(),
                Expr::NumberLiteral(2.0).into(),
            )
        );

        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Slash,
                TokenData::Number(2.0),
            ],
            Expr::Binary(
                BinOp::Div,
                Expr::NumberLiteral(1.0).into(),
                Expr::NumberLiteral(2.0).into(),
            )
        );

        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Star,
                TokenData::Number(2.0),
            ],
            Expr::Binary(
                BinOp::Mult,
                Expr::NumberLiteral(1.0).into(),
                Expr::NumberLiteral(2.0).into(),
            )
        );

        // left-associative on same operator
        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Star,
                TokenData::Number(2.0),
                TokenData::Star,
                TokenData::Number(3.0),
            ],
            Expr::Binary(
                BinOp::Mult,
                Expr::Binary(
                    BinOp::Mult,
                    Expr::NumberLiteral(1.0).into(),
                    Expr::NumberLiteral(2.0).into(),
                )
                .into(),
                Expr::NumberLiteral(3.0).into(),
            )
        );

        // mult takes precedence over add
        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Plus,
                TokenData::Number(2.0),
                TokenData::Star,
                TokenData::Number(3.0),
            ],
            Expr::Binary(
                BinOp::Add,
                Expr::NumberLiteral(1.0).into(),
                Expr::Binary(
                    BinOp::Mult,
                    Expr::NumberLiteral(2.0).into(),
                    Expr::NumberLiteral(3.0).into(),
                )
                .into(),
            )
        );
    }

    #[test]
    fn grouping() {
        assert_expr_parses!(
            tokens![
                TokenData::LeftParen,
                TokenData::Number(1.0),
                TokenData::Plus,
                TokenData::Number(2.0),
                TokenData::RightParen,
                TokenData::Star,
                TokenData::Number(3.0),
            ],
            Expr::Binary(
                BinOp::Mult,
                Expr::Binary(
                    BinOp::Add,
                    Expr::NumberLiteral(1.0).into(),
                    Expr::NumberLiteral(2.0).into(),
                )
                .into(),
                Expr::NumberLiteral(3.0).into(),
            )
        );
    }
}
