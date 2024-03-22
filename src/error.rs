#![allow(dead_code)]

use std::fmt::Display;

/// Represents a possible errored state that results from running the interpreter.
///
/// The interpreter can only return errors from one phase, because it won't procede to the next one
/// if there are errors.
///
/// The interpreter will try to produce as many scanner and parser errors at one time, but will stop
/// execution at the first runtime error.
#[derive(Debug)]
pub enum ErrorState {
    ScanErrs(Vec<Error>),
    ParseErrs(Vec<Error>),
    RuntimeErr(Error),
}

impl ErrorState {
    pub fn new_scanner_state() -> Self {
        Self::ScanErrs(vec![])
    }

    pub fn new_parser_state() -> Self {
        Self::ParseErrs(vec![])
    }

    pub fn runtime_error(e: String, lineno: u32) -> Self {
        Self::RuntimeErr(Error::runtime_error(e, lineno))
    }

    pub fn add(&mut self, e: Error) {
        match self {
            Self::ScanErrs(v) => v.push(e),
            Self::ParseErrs(v) => v.push(e),
            Self::RuntimeErr(_) => (), // can't update RuntimeError
        }
    }

    pub fn is_ok(&mut self) -> bool {
        match self {
            Self::ScanErrs(v) => v.is_empty(),
            Self::ParseErrs(v) => v.is_empty(),
            Self::RuntimeErr(_) => false,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    err: ErrorMsg,
    line: u32,
}

#[derive(Debug)]
enum ErrorMsg {
    Scan(String),
    Parse(String),
    Runtime(String),
}

impl Error {
    pub fn scan_error(msg: String, line: u32) -> Self {
        Self {
            line,
            err: ErrorMsg::Scan(msg),
        }
    }

    pub fn parse_error(msg: String, line: u32) -> Self {
        Self {
            line,
            err: ErrorMsg::Parse(msg),
        }
    }

    pub fn runtime_error(msg: String, line: u32) -> Self {
        Self {
            line,
            err: ErrorMsg::Runtime(msg),
        }
    }
}

impl Display for ErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorState::ScanErrs(errs) => {
                for e in errs {
                    writeln!(f, "{e}")?;
                }
            }
            ErrorState::ParseErrs(errs) => {
                for e in errs {
                    writeln!(f, "{e}")?;
                }
            }
            ErrorState::RuntimeErr(e) => write!(f, "{e}")?,
        }

        Ok(())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.line, self.err)
    }
}

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorMsg::Scan(msg) => write!(f, "scan error: {msg}"),
            ErrorMsg::Parse(msg) => write!(f, "parse error: {msg}"),
            ErrorMsg::Runtime(msg) => write!(f, "runtime error: {msg}"),
        }
    }
}
