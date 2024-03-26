use crate::error::{Error, ErrorState};
use crate::expr::{BinOp, Expr, ExprData, Program, Stmt, UnaryOp, Decl};
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

macro_rules! recurse_binary_expr {
    ( $self:expr, $left:expr, $recurse:expr, $( ( $token:path, $binop:path ) $(,)? )* ) => {{
        let Token { data, line } = $self.peek();
        let line = line.clone();

        match &data {
            $(
                $token => {
                    $self.next();
                    let right = $recurse;
                    Expr::new(
                        ExprData::Binary($binop, $left.clone().into(), right.into()),
                        line,
                    )
                }
            )*
            _ => break,
        }
    }}
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

    fn peek(&self) -> &Token {
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
            match self.declaration() {
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

    fn declaration(&mut self) -> Result<Decl, Error> {
        let decl = match &self.peek().data {
            Var => {
                self.next();

                let id = self.parse_identifier()?;

                // todo: allow chained equals
                self.expect(TokenData::Equal, "equal")?;

                let expr = self.parse_expression()?;

                Decl::VarDecl(id, expr)
            }

            _ => {
                let inner = self.statement()?;
                Decl::Stmt(inner)
            }
        };

        self.expect(TokenData::Semicolon, "semicolon")?;

        Ok(decl)
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        let stmt = match self.peek().data {
            // 'print' expr ;
            Print => {
                self.next();

                let inner = self.parse_expression()?;
                Stmt::Print(inner)
            }

            // bare expression ;
            _ => {
                let inner = self.parse_expression()?;
                Stmt::Expr(inner)
            }
        };

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
            expr = recurse_binary_expr!(
                self,
                expr,
                self.comparison()?,
                (TokenData::BangEqual, BinOp::Neq),
                (TokenData::EqualEqual, BinOp::Eq),
            );
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            expr = recurse_binary_expr!(
                self,
                expr,
                self.term()?,
                (TokenData::Greater, BinOp::Gt),
                (TokenData::GreaterEqual, BinOp::GtEq),
                (TokenData::Less, BinOp::Lt),
                (TokenData::LessEqual, BinOp::LtEq),
            );
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            expr = recurse_binary_expr!(
                self,
                expr,
                self.factor()?,
                (TokenData::Plus, BinOp::Add),
                (TokenData::Minus, BinOp::Sub),
            );
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        if self.is_at_end() {
            return Ok(expr);
        }

        loop {
            expr = recurse_binary_expr!(
                self,
                expr,
                self.unary()?,
                (TokenData::Slash, BinOp::Div),
                (TokenData::Star, BinOp::Mult),
            );
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        let Token { data, line } = self.peek();
        let line = *line;

        let expr = match &data {
            Minus => {
                self.next();
                let e = self.unary()?;
                Expr::new(ExprData::Unary(UnaryOp::Negative, e.into()), line)
            }
            Bang => {
                self.next();
                let e = self.unary()?;
                Expr::new(ExprData::Unary(UnaryOp::Inverse, e.into()), line)
            }
            _ => self.primary()?,
        };

        Ok(expr)
    }

    fn parse_identifier(&mut self) -> Result<Expr, Error> {
        let Token { data, line } = self.peek();
        let ident = match &data {
            Identifier(s) => {
                // clone the string out of the immutable borrow before modifying self
                let expr = Expr::new(ExprData::Identifier(s.clone()), *line);

                self.next();

                expr
            }
            _ => {
                return Err(Error::parse_error(
                    "expected valid identifier".into(),
                    *line,
                ));
            }
        };

        Ok(ident)
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        let Token { data, line } = self.peek();
        let ident = match &data {
            Identifier(s) => {
                // clone the string out of the immutable borrow before modifying self
                let expr = Expr::new(ExprData::Identifier(s.clone()), *line);

                self.next();

                expr
            }
            StringToken(s) => {
                // clone the string out of the immutable borrow before modifying self
                let expr = Expr::new(ExprData::StringLiteral(s.clone()), *line);

                self.next();

                expr
            }
            Number(n) => {
                // copy the literal out of the immutable borrow before modifying self
                let expr = Expr::new(ExprData::NumberLiteral(*n), *line);

                self.next();

                expr
            }
            True => {
                let expr = Expr::new(ExprData::True, *line);

                self.next();

                expr
            }
            False => {
                let expr = Expr::new(ExprData::False, *line);

                self.next();

                expr
            }
            Nil => {
                let expr = Expr::new(ExprData::Nil, *line);

                self.next();

                expr
            }
            LeftParen => {
                self.next(); // first move pointer past LeftParen

                let expr = self.equality()?;

                self.expect(TokenData::RightParen, "closing parens")?;

                expr
            }
            Eof => {
                return Err(Error::parse_error(
                    "unexpected end of file".to_string(),
                    *line,
                ))
            }
            t => {
                return Err(Error::parse_error(
                    format!("unexpected token: {t:?}"),
                    *line,
                ))
            }
        };

        Ok(ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{BinOp, Expr, ExprData, Stmt, UnaryOp};
    use crate::token::{Token, TokenData};
    use crate::tokens;

    use super::parse;

    // ignores line numbers
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

    // wraps ExprData in Expr
    macro_rules! e {
        ( $e:expr ) => {{
            Expr::new($e, 0)
        }};
    }

    #[test]
    fn literals() {
        assert_expr_parses!(tokens![TokenData::True], e!(ExprData::True));

        assert_expr_parses!(tokens![TokenData::False], e!(ExprData::False));

        assert_expr_parses!(tokens![TokenData::Nil], e!(ExprData::Nil));

        assert_expr_parses!(
            tokens![TokenData::Number(1.0)],
            e!(ExprData::NumberLiteral(1.0))
        );

        assert_expr_parses!(
            tokens![TokenData::StringToken("foo".to_string())],
            e!(ExprData::StringLiteral("foo".to_string()))
        );
    }

    #[test]
    fn unary() {
        assert_expr_parses!(
            tokens![TokenData::Bang, TokenData::False],
            e!(ExprData::Unary(
                UnaryOp::Inverse,
                e!(ExprData::False).into(),
            ))
        );

        assert_expr_parses!(
            tokens![TokenData::Minus, TokenData::False],
            e!(ExprData::Unary(
                UnaryOp::Negative,
                e!(ExprData::False).into()
            ))
        );
    }

    #[test]
    fn cmps() {
        assert_expr_parses!(
            tokens![TokenData::True, TokenData::Greater, TokenData::False,],
            e!(ExprData::Binary(
                BinOp::Gt,
                e!(ExprData::True).into(),
                e!(ExprData::False).into()
            ))
        );

        assert_expr_parses!(
            tokens![TokenData::True, TokenData::GreaterEqual, TokenData::False,],
            e!(ExprData::Binary(
                BinOp::GtEq,
                e!(ExprData::True).into(),
                e!(ExprData::False).into()
            ))
        );

        assert_expr_parses!(
            tokens![TokenData::True, TokenData::Less, TokenData::False,],
            e!(ExprData::Binary(
                BinOp::Lt,
                e!(ExprData::True).into(),
                e!(ExprData::False).into()
            ))
        );

        assert_expr_parses!(
            tokens![TokenData::True, TokenData::LessEqual, TokenData::False,],
            e!(ExprData::Binary(
                BinOp::LtEq,
                e!(ExprData::True).into(),
                e!(ExprData::False).into()
            ))
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
            e!(ExprData::Binary(
                BinOp::GtEq,
                e!(ExprData::Binary(
                    BinOp::LtEq,
                    e!(ExprData::NumberLiteral(1.0)).into(),
                    e!(ExprData::NumberLiteral(2.0)).into(),
                ))
                .into(),
                e!(ExprData::NumberLiteral(3.0)).into()
            ))
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
            e!(ExprData::Binary(
                BinOp::Add,
                e!(ExprData::NumberLiteral(1.0)).into(),
                e!(ExprData::NumberLiteral(2.0)).into(),
            ))
        );

        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Minus,
                TokenData::Number(2.0),
            ],
            e!(ExprData::Binary(
                BinOp::Sub,
                e!(ExprData::NumberLiteral(1.0)).into(),
                e!(ExprData::NumberLiteral(2.0)).into(),
            ))
        );

        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Slash,
                TokenData::Number(2.0),
            ],
            e!(ExprData::Binary(
                BinOp::Div,
                e!(ExprData::NumberLiteral(1.0)).into(),
                e!(ExprData::NumberLiteral(2.0)).into(),
            ))
        );

        assert_expr_parses!(
            tokens![
                TokenData::Number(1.0),
                TokenData::Star,
                TokenData::Number(2.0),
            ],
            e!(ExprData::Binary(
                BinOp::Mult,
                e!(ExprData::NumberLiteral(1.0)).into(),
                e!(ExprData::NumberLiteral(2.0)).into(),
            ))
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
            e!(ExprData::Binary(
                BinOp::Mult,
                e!(ExprData::Binary(
                    BinOp::Mult,
                    e!(ExprData::NumberLiteral(1.0)).into(),
                    e!(ExprData::NumberLiteral(2.0)).into(),
                ))
                .into(),
                e!(ExprData::NumberLiteral(3.0)).into(),
            ))
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
            e!(ExprData::Binary(
                BinOp::Add,
                e!(ExprData::NumberLiteral(1.0)).into(),
                e!(ExprData::Binary(
                    BinOp::Mult,
                    e!(ExprData::NumberLiteral(2.0)).into(),
                    e!(ExprData::NumberLiteral(3.0)).into(),
                ))
                .into(),
            ))
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
            e!(ExprData::Binary(
                BinOp::Mult,
                e!(ExprData::Binary(
                    BinOp::Add,
                    e!(ExprData::NumberLiteral(1.0)).into(),
                    e!(ExprData::NumberLiteral(2.0)).into(),
                ))
                .into(),
                e!(ExprData::NumberLiteral(3.0)).into(),
            ))
        );
    }
}
