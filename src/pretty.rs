use crate::expr::{BinOp, Expr, ExprData, UnaryOp};

macro_rules! indent {
    ( $v:expr, $n:expr) => {{
        println!("{}{}", " ".repeat($n), $v);
    }};
}
macro_rules! pretty {
    ( $s:expr, $left:expr, $right:expr, $indent:expr) => {{
        $left.pretty_recur($indent + 4);
        println!("{}{}", " ".repeat($indent), $s);
        $right.pretty_recur($indent + 4);
    }};
}

impl Expr {
    pub fn pretty(&self) {
        self.data.pretty_recur(0)
    }

    pub fn pretty_recur(&self, indent: usize) {
        self.data.pretty_recur(indent)
    }
}

impl ExprData {
    pub fn pretty_recur(&self, indent: usize) {
        match self {
            Self::Binary(op, left, right) => {
                let op = match op {
                    BinOp::Eq => "==",
                    BinOp::Neq => "!=",
                    BinOp::Gt => ">",
                    BinOp::GtEq => ">=",
                    BinOp::Lt => "<",
                    BinOp::LtEq => "<=",
                    BinOp::Add => "+",
                    BinOp::Sub => "-",
                    BinOp::Div => "/",
                    BinOp::Mult => "*",
                };
                pretty!(op, left, right, indent)
            }

            Self::Unary(op, e) => {
                let op = match op {
                    UnaryOp::Negative => "-",
                    UnaryOp::Inverse => "!",
                };
                println!("{}{}", " ".repeat(indent), op);
                e.pretty_recur(indent + 4);
            }

            Self::Identifier(s) => indent!(format!("{s}"), indent),
            Self::StringLiteral(s) => indent!(format!("\"{s}\""), indent),
            Self::NumberLiteral(n) => indent!(format!("{n}"), indent),
            Self::True => indent!("true", indent),
            Self::False => indent!("false", indent),
            Self::Nil => indent!("nil", indent),
        }
    }
}
