use std::iter::Peekable;

use crate::error::{Error, ErrorState};
use crate::token::{Token, TokenData};

/// mutates feed if the condition is met to consume the second character
/// condition : result ? else
fn double_char_ternary<I: Iterator<Item = char>>(
    feed: &mut Peekable<I>,
    conditional_match: char,
    if_true: TokenData,
    if_false: TokenData,
) -> TokenData {
    if let Some(&c) = feed.peek() {
        if c == conditional_match {
            feed.next();
            return if_true;
        }
    }

    if_false
}

// doesn't consume final character
// todo: doesn't handle newlines in string literals
fn consume_until<I: Iterator<Item = char>>(
    feed: &mut Peekable<I>,
    ending_char: char,
) -> Result<String, Error> {
    let chars = consume_while(feed, |c| c != ending_char)?;

    Ok(chars.iter().collect())
}

fn consume_while<I: Iterator<Item = char>, F: Fn(char) -> bool>(
    feed: &mut Peekable<I>,
    condition: F,
) -> Result<Vec<char>, Error> {
    let mut acc = Vec::new();

    while let Some(&c) = feed.peek() {
        if condition(c) {
            acc.push(c);
            feed.next();
        } else {
            break;
        }
    }

    Ok(acc)
}

pub fn scan(text: &str, starting_line: u32) -> Result<Vec<Token>, ErrorState> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut err_state = ErrorState::new_scanner_state();

    let mut lineno = starting_line;
    let mut feed = text.chars().peekable();

    loop {
        let next = feed.next();
        if next.is_none() {
            tokens.push(Token::new(TokenData::Eof, lineno));
            break;
        }

        let c = next.unwrap();
        match c {
            // Unambiguous single character
            '(' => tokens.push(Token::new(TokenData::LeftParen, lineno)),
            ')' => tokens.push(Token::new(TokenData::RightParen, lineno)),
            '{' => tokens.push(Token::new(TokenData::LeftBrace, lineno)),
            '}' => tokens.push(Token::new(TokenData::RightBrace, lineno)),
            ',' => tokens.push(Token::new(TokenData::Comma, lineno)),
            '.' => tokens.push(Token::new(TokenData::Dot, lineno)),
            '-' => tokens.push(Token::new(TokenData::Minus, lineno)),
            '+' => tokens.push(Token::new(TokenData::Plus, lineno)),
            ';' => tokens.push(Token::new(TokenData::Semicolon, lineno)),
            '*' => tokens.push(Token::new(TokenData::Star, lineno)),

            // Single or double character operators
            '!' => {
                let t = double_char_ternary(&mut feed, '=', TokenData::BangEqual, TokenData::Bang);
                tokens.push(Token::new(t, lineno));
            }
            '=' => {
                let t =
                    double_char_ternary(&mut feed, '=', TokenData::EqualEqual, TokenData::Equal);
                tokens.push(Token::new(t, lineno));
            }
            '>' => {
                let t = double_char_ternary(
                    &mut feed,
                    '=',
                    TokenData::GreaterEqual,
                    TokenData::Greater,
                );
                tokens.push(Token::new(t, lineno));
            }
            '<' => {
                let t = double_char_ternary(&mut feed, '=', TokenData::LessEqual, TokenData::Less);
                tokens.push(Token::new(t, lineno));
            }

            // Slashes & comments
            '/' => {
                if let Some('/') = feed.peek() {
                    // consume second slash
                    feed.next();

                    // discard comment string
                    if let Err(e) = consume_until(&mut feed, '\n') {
                        err_state.add(e);
                        break;
                        // todo - is this recoverable?
                    }
                } else {
                    tokens.push(Token::new(TokenData::Slash, lineno));
                }
            }

            // string literals
            '"' => {
                let literal = match consume_until(&mut feed, '"') {
                    Ok(v) => v,
                    Err(e) => {
                        err_state.add(e);
                        break;
                        // todo - is this recoverable?
                    }
                };

                tokens.push(Token::new(TokenData::StringToken(literal), lineno));

                // consume closing quote
                feed.next();
            }

            // ignore whitespace
            ' ' | '\r' | '\t' => (),

            // newline
            '\n' => lineno += 1,

            // fallthrough: need to call a fn on c
            c => {
                // todo: should bail out of number parsing if the char after the `.` is not a digit
                if c.is_ascii_digit() {
                    let mut acc = vec![c];
                    let part_two =
                        match consume_while(&mut feed, |c| c.is_ascii_digit() || c == '.') {
                            Ok(v) => v,
                            Err(e) => {
                                err_state.add(e);
                                break;
                                // todo - is this recoverable?
                            }
                        };

                    acc.extend(part_two.iter());
                    let word = acc.iter().collect::<String>();

                    match word.parse() {
                        Ok(n) => tokens.push(Token::new(TokenData::Number(n), lineno)),
                        Err(e) => {
                            err_state.add(Error::scan_error(
                                format!("invalid number literal: {word}, {e}"),
                                lineno,
                            ));
                        }
                    }
                } else if is_word(c) {
                    let mut acc = vec![c];
                    let part_two = match consume_while(&mut feed, is_word) {
                        Ok(v) => v,
                        Err(e) => {
                            err_state.add(e);
                            break;
                            // todo - is this recoverable?
                        }
                    };

                    acc.extend(part_two.iter());

                    let word = acc.iter().collect::<String>();
                    let keyword = match match_keyword(word) {
                        Ok(v) => v,
                        Err(e) => {
                            err_state.add(e);
                            break;
                            // todo - is this recoverable?
                        }
                    };

                    tokens.push(Token::new(keyword, lineno));
                } else {
                    // todo: multiple errors
                    err_state.add(Error::scan_error(
                        format!("unexpected character: {c}"),
                        lineno,
                    ));
                    break;
                }
            }
        }
    }

    Ok(tokens)
}

// keywords or identifier literals
fn is_word(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_'
}

fn match_keyword(s: String) -> Result<TokenData, Error> {
    let t = match s.as_str() {
        "and" => TokenData::And,
        "class" => TokenData::Class,
        "else" => TokenData::Else,
        "false" => TokenData::False,
        "fun" => TokenData::Fun,
        "for" => TokenData::For,
        "if" => TokenData::If,
        "nil" => TokenData::Nil,
        "or" => TokenData::Or,
        "print" => TokenData::Print,
        "return" => TokenData::Return,
        "super" => TokenData::Super,
        "this" => TokenData::This,
        "true" => TokenData::True,
        "var" => TokenData::Var,
        "while" => TokenData::While,

        _ => TokenData::Identifier(s),
    };

    Ok(t)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::token::{Token, TokenData::*};
    use crate::tokens;

    use super::scan;

    #[test]
    fn macro_test() {
        assert_eq!(
            tokens![(If, 0), (Else, 1)],
            vec![
                Token::new(If, 0), Token::new(Else, 1),
            ]
        );
    }

    macro_rules! assert_tokens {
        ( $s:literal, $t:expr ) => {{
            let actual_tokens = scan($s, 0).unwrap();
            assert_eq!(actual_tokens, $t);
        }};
    }

    #[test]
    fn singles() {
        assert_tokens!(
            "( { } )
            , . - + ; / *",
            tokens![
                (LeftParen, 0), (LeftBrace, 0), (RightBrace, 0), (RightParen, 0),
                (Comma, 1), (Dot, 1), (Minus, 1), (Plus, 1), (Semicolon, 1), (Slash, 1), (Star, 1),
                (Eof, 1),
            ]
        );
    }

    #[test]
    fn doubles() {
        assert_tokens!(
            "! !=
            = ==
            > >=
            < <=",
            tokens![
                (Bang, 0),    (BangEqual, 0),
                (Equal, 1),   (EqualEqual, 1),
                (Greater, 2), (GreaterEqual, 2),
                (Less, 3),    (LessEqual, 3),
                (Eof, 3),
            ]
        );
    }

    #[test]
    fn literals() {
        assert_tokens!(
            "id
            \"literal\"
            123
            4.0",
            tokens![
                (Identifier("id".to_string()), 0),
                (StringToken("literal".to_string()), 1),
                (Number(123.0), 2),
                (Number(4.0), 3),
                (Eof, 3),
            ]
        );
    }

    #[test]
    fn keywords() {
        assert_tokens!(
            "if else
            for while
            true false
            class this super
            and or
            print
            fun return
            var nil",
            tokens![
                (If, 0),    (Else, 0),
                (For, 1),   (While, 1),
                (True, 2) , (False, 2),
                (Class, 3), (This, 3),  (Super, 3),
                (And, 4),   (Or, 4),
                (Print, 5),
                (Fun, 6),   (Return, 6),
                (Var, 7),   (Nil, 7),
                (Eof, 7)
            ]
        );
    }

    #[test]
    fn comments() {
        assert_tokens!(
            "/
            // ignored",
            tokens![
                (Slash, 0),
                (Eof, 1),
            ]
        );
    }
}
