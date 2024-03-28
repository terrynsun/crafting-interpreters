use std::fmt::Display;

use crate::error::ErrorState;
use crate::expr::{BinOp, Expr, ExprData, UnaryOp};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(v) => write!(f, "{v}"),
            Value::String(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Result<Value, ErrorState> {
        self.data.eval(self.line)
    }
}

impl ExprData {
    pub fn eval(&self, line: u32) -> Result<Value, ErrorState> {
        match self {
            Self::Binary(op, left_expr, right_expr) => {
                let left_val = left_expr.eval()?;
                let right_val = right_expr.eval()?;

                match op {
                    BinOp::Eq => Ok(Value::Boolean(left_val == right_val)),
                    BinOp::Neq => Ok(Value::Boolean(left_val != right_val)),

                    BinOp::Gt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a > b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::GtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a >= b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Lt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a < b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::LtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a <= b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }

                    BinOp::Add => {
                        if let (Value::Number(a), Value::Number(b)) = (&left_val, &right_val) {
                            Ok(Value::Number(a + b))
                        } else if let (Value::String(a), Value::String(b)) = (left_val, right_val) {
                            Ok(Value::String(format!("{a}{b}")))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only add numbers or strings".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Sub => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a - b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only subtract numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a / b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only divide numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Mult => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a * b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only multiply numbers".into(),
                                line,
                            ))
                        }
                    }
                }
            }

            Self::Unary(op, e) => {
                let val = e.eval()?;
                match op {
                    UnaryOp::Negative => {
                        if let Value::Number(n) = val {
                            Ok(Value::Number(-n))
                        } else {
                            Err(ErrorState::runtime_error(
                                "- can only be applied to numbers".into(),
                                line,
                            ))
                        }
                    }
                    UnaryOp::Inverse => {
                        if let Value::Boolean(b) = val {
                            Ok(Value::Boolean(!b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "! can only be applied to numbers".into(),
                                line,
                            ))
                        }
                    }
                }
            }

            Self::Identifier(_) => Err(ErrorState::runtime_error(
                "! can only be applied to numbers".into(),
                line,
            )),
            Self::StringLiteral(s) => Ok(Value::String(s.clone())),
            Self::NumberLiteral(n) => Ok(Value::Number(*n)),
            Self::True => Ok(Value::Boolean(true)),
            Self::False => Ok(Value::Boolean(false)),
            Self::Nil => Ok(Value::Nil),
        }
    }
}
