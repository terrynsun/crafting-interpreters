use crate::expr::Expr;

macro_rules! indent {
    ( $v:expr, $n:expr) => {{
        println!("{}{}", " ".repeat($n), $v);
    }};
}
macro_rules! pretty {
    ( $s:literal, $left:expr, $right:expr, $indent:expr) => {{
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
            Expr::Eq(left, right) => pretty!("==", left, right, indent),
            Expr::Neq(left, right) => pretty!("!=", left, right, indent),

            Expr::Gt(left, right) => pretty!("<", left, right, indent),
            Expr::GtEq(left, right) => pretty!("<", left, right, indent),
            Expr::Lt(left, right) => pretty!("<", left, right, indent),
            Expr::LtEq(left, right) => pretty!("<", left, right, indent),

            Expr::Add(left, right) => pretty!("+", left, right, indent),
            Expr::Sub(left, right) => pretty!("-", left, right, indent),
            Expr::Div(left, right) => pretty!("*", left, right, indent),
            Expr::Mult(left, right) => pretty!("/", left, right, indent),

            Expr::Negative(e) => {
                println!("{}-", " ".repeat(indent));
                e.pretty_recur(indent + 4);
            }
            Expr::Inverse(e) => {
                println!("{}!", " ".repeat(indent));
                e.pretty_recur(indent + 4);
            }

            Expr::NumberLiteral(n) => indent!(format!("{n}"), indent),
            Expr::StringLiteral(s) => indent!(format!("{s}"), indent),
            Expr::TrueExpr => indent!("true", indent),
            Expr::FalseExpr => indent!("false", indent),
            Expr::NilExpr => indent!("nil", indent),
        }
    }
}
