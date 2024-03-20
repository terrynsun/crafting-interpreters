use std::rc::Rc;

// expression     → literal
//                | unary
//                | binary
//                | grouping ;
//
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

// Precedence: (lowest = highest)
//
// Equality (== !=)
// Comparison (> >= < <=)
// Term (- +)
// Factor (/ *)
// Unary (! -)
//
// expression     → equality
// equality       → comparison ( (!= | ==) comparison )*
// comparison     → term (( "<>" etc ) term)*
// term           → factor (( "-" | "+" ) factor)*
// factor         → unary ( ("/" | "*") unary )*
// unary          → ("!" | "-") unary | primary
// primary        → literal | "(" expression ")"

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq(Rc<Expr>, Rc<Expr>),
    Neq(Rc<Expr>, Rc<Expr>),

    Gt(Rc<Expr>, Rc<Expr>),
    GtEq(Rc<Expr>, Rc<Expr>),
    Lt(Rc<Expr>, Rc<Expr>),
    LtEq(Rc<Expr>, Rc<Expr>),

    Add(Rc<Expr>, Rc<Expr>),
    Sub(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Mult(Rc<Expr>, Rc<Expr>),

    Negative(Rc<Expr>),
    Inverse(Rc<Expr>),

    Literal(LiteralExpr),
}

#[derive(Clone, PartialEq)]
pub enum LiteralExpr {
    NumberLiteral(f32),
    StringLiteral(String),
    True,
    False,
    Nil,
}
