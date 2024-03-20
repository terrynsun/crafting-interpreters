#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub data: TokenData,
    line: u32,
}

impl Token {
    pub fn new(t: TokenData, line: u32) -> Self {
        Self { data: t, line }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenData {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    StringToken(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! tokens {
        ( $( ($t:expr, $l:literal) ),* $(,)? ) => {
            {
                let mut v = Vec::new();
                $(
                    v.push(Token::new($t, $l));
                )*
                v
            }
        };
        ( $( $t:expr $(,)? )* ) => {
            {
                let mut v = Vec::new();
                $(
                    v.push(Token::new($t, 0));
                )*
                v
            }
        };
    }
}
