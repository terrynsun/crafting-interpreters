use crate::expr::{Expr, BinOp, UnaryOp};

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
    pub fn eval(&self) -> Result<Value, String> {
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
                            Err("runtime error: can only compare numbers".into())
                        }
                    }
                    BinOp::GtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a >= b))
                        } else {
                            Err("runtime error: can only compare numbers".into())
                        }
                    }
                    BinOp::Lt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a < b))
                        } else {
                            Err("runtime error: can only compare numbers".into())
                        }
                    }
                    BinOp::LtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a <= b))
                        } else {
                            Err("runtime error: can only compare numbers".into())
                        }
                    }

                    BinOp::Add => {
                        if let (Value::Number(a), Value::Number(b)) = (&left_val, &right_val) {
                            Ok(Value::Number(a + b))
                        } else if let (Value::String(a), Value::String(b)) = (left_val, right_val) {
                            Ok(Value::String(format!("{a}{b}")))
                        } else {
                            Err("runtime error: can only add numbers or strings".into())
                        }
                    }
                    BinOp::Sub => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a - b))
                        } else {
                            Err("runtime error: can only subtract numbers".into())
                        }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a / b))
                        } else {
                            Err("runtime error: can only divide numbers".into())
                        }
                    }
                    BinOp::Mult => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a * b))
                        } else {
                            Err("runtime error: can only multiply numbers".into())
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
                            Err("runtime error: - can only be applied to numbers".into())
                        }
                    },
                    UnaryOp::Inverse => {
                        if let Value::Boolean(b) = val {
                            Ok(Value::Boolean(!b))
                        } else {
                            Err("runtime error: ! can only be applied to boolean expressions".into())
                        }
                    },
                }
            }

            Expr::NumberLiteral(n) => Ok(Value::Number(*n)),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expr::True => Ok(Value::Boolean(true)),
            Expr::False => Ok(Value::Boolean(false)),
            Expr::Nil => Ok(Value::Nil),
        }
    }
}
