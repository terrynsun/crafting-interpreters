#![allow(dead_code)]

pub struct Error {
    line: u32,
    err: ErrType,
}

enum ErrType {
    Other(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.line, self.err)
    }
}

impl std::fmt::Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrType::Other(msg) => write!(f, "{msg}")
        }
    }
}
