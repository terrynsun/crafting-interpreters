use crate::expr::{BinOp, Expr, UnaryOp};

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
        self.pretty_recur(0)
    }

    pub fn pretty_recur(&self, indent: usize) {
        match self {
            Expr::Binary(op, left, right) => {
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

            Expr::Unary(op, e) => {
                let op = match op {
                    UnaryOp::Negative => "-",
                    UnaryOp::Inverse => "!",
                };
                println!("{}{}", " ".repeat(indent), op);
                e.pretty_recur(indent + 4);
            }

            Expr::Identifier(s) => indent!(format!("{s}"), indent),
            Expr::StringLiteral(s) => indent!(format!("\"{s}\""), indent),
            Expr::NumberLiteral(n) => indent!(format!("{n}"), indent),
            Expr::True => indent!("true", indent),
            Expr::False => indent!("false", indent),
            Expr::Nil => indent!("nil", indent),
        }
    }
}
