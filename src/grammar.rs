use std::rc::Rc;

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
pub struct Expr {
    pub data: ExprData,
    pub line: u32,
}

impl Expr {
    pub fn new(data: ExprData, line: u32) -> Self {
        Self { data, line }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprData {
    Assignment(Rc<Expr>, Rc<Expr>),

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

// program        → declaration* EOF ;

// declaration    → varDecl
//                | statement ;
//
// varDecl        → "var" IDENTIFIER ( '=' expression ) ? ;
//
// statement      → exprStmt
//                | ifStmt
//                | printStmt
//                | block ;
//
// ifStmt         → "if" "(" expression ")" statement
//                  ( "else" statement )? ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// block          → "{" declaration* "}" ;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Vec<Decl>),

    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    // The first Expr must be an identifier
    VarDecl(Expr, Expr),

    Stmt(Stmt),
}

pub type Program = Vec<Decl>;
