#![allow(dead_code)]

#[derive(Debug)]
pub struct Error {
    err: ErrType,
    line: u32,
}

impl Error {
    pub fn new_with_msg(msg: String, line: u32) -> Self {
        Self {
            line,
            err: ErrType::Other(msg),
        }
    }
}

#[derive(Debug)]
enum ErrType {
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.line, self.err)
    }
}

impl std::fmt::Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrType::Other(msg) => write!(f, "{msg}"),
        }
    }
}
