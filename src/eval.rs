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
    pub fn eval(&self) -> Value {
        match self {
            Expr::Binary(op, left_expr, right_expr) => {
                let left_val = left_expr.eval();
                let right_val = right_expr.eval();

                match op {
                    BinOp::Eq => Value::Boolean(left_val == right_val),
                    BinOp::Neq => Value::Boolean(left_val != right_val),

                    BinOp::Gt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Boolean(a > b)
                        } else {
                            panic!("runtime error: can only compare numbers")
                        }
                    }
                    BinOp::GtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Boolean(a >= b)
                        } else {
                            panic!("runtime error: can only compare numbers")
                        }
                    }
                    BinOp::Lt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Boolean(a < b)
                        } else {
                            panic!("runtime error: can only compare numbers")
                        }
                    }
                    BinOp::LtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Boolean(a <= b)
                        } else {
                            panic!("runtime error: can only compare numbers")
                        }
                    }

                    BinOp::Add => {
                        if let (Value::Number(a), Value::Number(b)) = (&left_val, &right_val) {
                            Value::Number(a + b)
                        } else if let (Value::String(a), Value::String(b)) = (left_val, right_val) {
                            Value::String(format!("{a}{b}"))
                        } else {
                            panic!("runtime error: can only add numbers or strings")
                        }
                    }
                    BinOp::Sub => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Number(a - b)
                        } else {
                            panic!("runtime error: can only subtract numbers")
                        }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Number(a / b)
                        } else {
                            panic!("runtime error: can only divide numbers")
                        }
                    }
                    BinOp::Mult => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Value::Number(a * b)
                        } else {
                            panic!("runtime error: can only multiply numbers")
                        }
                    }
                }
            }

            Expr::Unary(op, e) => {
                let val = e.eval();
                match op {
                    UnaryOp::Negative => {
                        if let Value::Number(n) = val {
                            Value::Number(-n)
                        } else {
                            panic!("runtime error: - can only be applied to numbers");
                        }
                    },
                    UnaryOp::Inverse => {
                        if let Value::Boolean(b) = val {
                            Value::Boolean(!b)
                        } else {
                            panic!("runtime error: ! can only be applied to boolean expressions");
                        }
                    },
                }
            }

            Expr::NumberLiteral(n) => Value::Number(*n),
            Expr::StringLiteral(s) => Value::String(s.clone()),
            Expr::True => Value::Boolean(true),
            Expr::False => Value::Boolean(false),
            Expr::Nil => Value::Nil,
        }
    }
}
