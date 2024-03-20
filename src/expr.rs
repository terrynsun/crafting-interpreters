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
    Binary(BinOp, Rc<Expr>, Rc<Expr>),
    Unary(UnaryOp, Rc<Expr>),

    NumberLiteral(f32),
    Identifier(String),
    StringLiteral(String),

    True,
    False,
    Nil,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinOp {
    Eq,
    Neq,
    Gt,
    GtEq,
    Lt,
    LtEq,

    Add,
    Sub,
    Div,
    Mult,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Negative,
    Inverse,
}

// program        → statement* EOF ;
//
// statement      → exprStmt
//                | printStmt ;
//
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    PrintStmt(Expr),
}

pub type Program = Vec<Stmt>;
