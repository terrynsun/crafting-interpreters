use crate::error::ErrorState;
use crate::expr::{BinOp, Expr, UnaryOp};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
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
        match self {
            Expr::Binary(op, left_expr, right_expr) => {
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
                                0,
                            ))
                        }
                    }
                    BinOp::GtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a >= b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                0,
                            ))
                        }
                    }
                    BinOp::Lt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a < b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                0,
                            ))
                        }
                    }
                    BinOp::LtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a <= b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                0,
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
                                0,
                            ))
                        }
                    }
                    BinOp::Sub => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a - b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only subtract numbers".into(),
                                0,
                            ))
                        }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a / b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only divide numbers".into(),
                                0,
                            ))
                        }
                    }
                    BinOp::Mult => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a * b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only multiply numbers".into(),
                                0,
                            ))
                        }
                    }
                }
            }

            Expr::Unary(op, e) => {
                let val = e.eval()?;
                match op {
                    UnaryOp::Negative => {
                        if let Value::Number(n) = val {
                            Ok(Value::Number(-n))
                        } else {
                            Err(ErrorState::runtime_error(
                                "- can only be applied to numbers".into(),
                                0,
                            ))
                        }
                    }
                    UnaryOp::Inverse => {
                        if let Value::Boolean(b) = val {
                            Ok(Value::Boolean(!b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "! can only be applied to numbers".into(),
                                0,
                            ))
                        }
                    }
                }
            }

            Expr::Identifier(_) => Err(ErrorState::runtime_error(
                "! can only be applied to numbers".into(),
                0,
            )),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expr::NumberLiteral(n) => Ok(Value::Number(*n)),
            Expr::True => Ok(Value::Boolean(true)),
            Expr::False => Ok(Value::Boolean(false)),
            Expr::Nil => Ok(Value::Nil),
        }
    }
}
