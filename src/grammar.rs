use std::rc::Rc;

// Precedence:
// Lowest in list = highest priority. The recursive descent parser starts at the top and parses
// downward, so remember that lowest things on this list have their logic executed first.
//
// expression     → assignment ;
//
// assignment     → IDENTIFIER "=" assignment | equality ;
//
// equality       → comparison ( (!= | ==) comparison )* ;
// comparison     → term (( "<>" etc ) term)* ;
// term           → factor (( "-" | "+" ) factor)* ;
// factor         → unary ( ("/" | "*") unary )* ;
// unary          → ("!" | "-") unary | call ;
//
// call           → primary ( "(" arguments? ")" )* ;
// arguments      → expression ( "," expression )* ;
//
// primary        → literal | "(" expression ")" ;

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
    FnCall(Rc<Expr>, Vec<Expr>),

    // todo: allow chained assignments
    Assignment(Rc<Expr>, Rc<Expr>),

    Binary(BinOp, Rc<Expr>, Rc<Expr>),
    Unary(UnaryOp, Rc<Expr>),

    NumberLiteral(f64),
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
//                | funDecl
//                | statement ;
//
// varDecl        → "var" IDENTIFIER ( '=' expression ) ? ;
//
// funDecl        → "fun" function
// function       → IDENTIFIER "(" parameters? ")" block ;
// arguments      → IDENTIFIER ( "," IDENTIFIER )* ;
//
// statement      → exprStmt
//                | printStmt
//                | block
//                | ifStmt
//                | whileStmt ;
//
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// block          → "{" declaration* "}" ;
//
// ifStmt         → "if" "(" expression ")" statement
//                  ( "else" statement )? ;
//
// whileStmt      → "while" "(" expression ")" statement ;
//
// forStmt        → "for" "(" declaration ; expression ; statement ")" statement ;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Vec<Decl>),

    // condition, then, else
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),

    // forStmts desugar into `Stmt::While`s
    // condition, body
    While(Expr, Box<Stmt>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    // The first Expr must be an identifier
    // (lvalue, rvalue)
    VarDecl(Expr, Expr),

    // The first Expr must be an identifier
    // name (Expr::Identifier), parameters (identifiers), body (Stmt::Block)
    FunDecl(Expr, Vec<String>, Stmt),

    Stmt(Stmt),
}

pub type Program = Vec<Decl>;
