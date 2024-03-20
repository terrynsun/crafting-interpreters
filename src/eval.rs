use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

impl Expr {
    pub fn eval(&self) -> Value {
        match self {
            Expr::Eq(_, _) => todo!(),
            Expr::Neq(_, _) => todo!(),

            Expr::Gt(_, _) => todo!(),
            Expr::GtEq(_, _) => todo!(),
            Expr::Lt(_, _) => todo!(),
            Expr::LtEq(_, _) => todo!(),

            Expr::Add(left, right) => {
                if let (Value::Number(a), Value::Number(b)) = (left.eval(), right.eval()) {
                    Value::Number(a + b)
                } else if let (Value::String(a), Value::String(b)) = (left.eval(), right.eval()) {
                    Value::String(format!("{a}{b}"))
                } else {
                    panic!("runtime error: can only add numbers or strings")
                }
            }

            Expr::Sub(left, right) => {
                if let (Value::Number(a), Value::Number(b)) = (left.eval(), right.eval()) {
                    Value::Number(a - b)
                } else {
                    panic!("runtime error: can only divide numbers")
                }
            }

            Expr::Div(left, right) => {
                if let (Value::Number(a), Value::Number(b)) = (left.eval(), right.eval()) {
                    Value::Number(a / b)
                } else {
                    panic!("runtime error: can only divide numbers")
                }
            }
            Expr::Mult(left, right) => {
                if let (Value::Number(a), Value::Number(b)) = (left.eval(), right.eval()) {
                    Value::Number(a*b)
                } else {
                    panic!("runtime error: can only multiply numbers")
                }
            }

            Expr::Negative(e) => {
                if let Value::Number(n) = e.eval() {
                    Value::Number(-n)
                } else {
                    panic!("runtime error: - can only be applied to numbers");
                }
            }
            Expr::Inverse(e) => {
                if let Value::Boolean(b) = e.eval() {
                    Value::Boolean(!b)
                } else {
                    panic!("runtime error: ! can only be applied to boolean expressions");
                }
            }

            Expr::NumberLiteral(n) => Value::Number(*n),
            Expr::StringLiteral(s) => Value::String(s.clone()),
            Expr::TrueExpr => Value::Boolean(true),
            Expr::FalseExpr => Value::Boolean(false),
            Expr::NilExpr => Value::Nil,
        }
    }
}
