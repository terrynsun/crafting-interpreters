use std::fmt::Display;
use std::rc::Rc;

use crate::error::ErrorState;
use crate::exec::Environment;
use crate::grammar::{BinOp, Expr, ExprData, Stmt, UnaryOp};

/// Represents a single value in lox.
#[derive(Clone, Debug)]
pub enum Value {
    NativeFn(NativeFn),
    LoxFn(LoxFn),

    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Clone)]
pub struct NativeFn {
    // array of identifiers
    arg_names: Vec<String>,

    body: Rc<dyn Fn(&mut Environment, Vec<Value>) -> Result<Value, ErrorState>>,
}

impl NativeFn {
    pub fn new(
        arg_names: Vec<String>,
        body: Rc<dyn Fn(&mut Environment, Vec<Value>) -> Result<Value, ErrorState>>,
    ) -> Self {
        Self { arg_names, body }
    }

    pub fn call(&self, env: &mut Environment, args: Vec<Value>) -> Result<Value, ErrorState> {
        (self.body)(env, args)
    }
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

#[derive(Clone, Debug)]
pub struct LoxFn {
    // array of identifiers
    arg_names: Vec<String>,

    body: Stmt,
}

impl LoxFn {
    fn arity(&self) -> u32 {
        self.arg_names.len() as u32
    }
}

impl LoxFn {
    pub fn new(arg_names: Vec<String>, body: Stmt) -> Self {
        Self { arg_names, body }
    }

    pub fn call(&self, env: &mut Environment, _args: Vec<Value>) -> Result<Value, ErrorState> {
        env.open_scope();

        //let value = (self.body)(env)?;

        env.pop_scope();

        // todo!
        Ok(Value::Nil)
    }
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::NativeFn(_) | Value::LoxFn(_) => true,

            Value::Number(n) => *n != 0.0,

            Value::String(s) => s != "",

            Value::Boolean(b) => *b,

            Value::Nil => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // todo!: better display
            Value::NativeFn(v) => write!(f, "{v:?}"),
            Value::LoxFn(v) => write!(f, "{v:?}"),
            Value::Number(v) => write!(f, "{v}"),
            Value::String(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::Nil => write!(f, "nil"),
        }
    }
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
    /// Evaluates an expression to a value. Takes in an environment to evaluate other global and
    /// local variables.
    pub fn eval(&self, state: &mut Environment) -> Result<Value, ErrorState> {
        self.data.eval(self.line, state)
    }
}

impl ExprData {
    pub fn eval(&self, line: u32, state: &mut Environment) -> Result<Value, ErrorState> {
        match self {
            Self::FnCall(callee, args) => {
                let callee_fn = callee.eval(state)?;

                // todo: can we map this?
                //let arg_values: Vec<Result<Value, ErrorState>> = args.iter().map(|a| a.eval(state)).collect();
                let mut arg_values = vec![];
                for a in args {
                    arg_values.push(a.eval(state)?);
                }

                // todo: check arity
                state.open_scope();

                let value = match callee_fn {
                    Value::NativeFn(f) => f.call(state, arg_values),
                    //Value::LoxFn(f) => f.call(state, arg_values)?,
                    _ => panic!("not a valid call target"),
                };

                state.pop_scope();

                value
            }
            Self::Assignment(lvalue, rvalue) => {
                let val = rvalue.eval(state)?;

                // todo: modified from exec's decl. needed to_string and clone. why?
                match &lvalue.data {
                    ExprData::Identifier(s) => {
                        if state.contains(s) {
                            if let Err(()) = state.update(s.to_string(), val.clone()) {
                                // todo - duplicated logic/error message. In _theory_, we shouldn't
                                // reach this one because we already checked `contains`.
                                return Err(ErrorState::runtime_error(
                                    format!("Undefined variable \"{s}\"").into(),
                                    line,
                                ));
                            }
                        } else {
                            return Err(ErrorState::runtime_error(
                                format!("Undefined variable \"{s}\"").into(),
                                line,
                            ));
                        }
                    }
                    _ => {
                        return Err(ErrorState::runtime_error(
                            "expected identifier".to_string(),
                            line,
                        ))
                    }
                }

                Ok(val)
            }

            Self::Binary(op, left_expr, right_expr) => {
                let left_val = left_expr.eval(state)?;
                let right_val = right_expr.eval(state)?;

                match op {
                    BinOp::Eq => Ok(Value::Boolean(left_val == right_val)),
                    BinOp::Neq => Ok(Value::Boolean(left_val != right_val)),

                    BinOp::Gt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a > b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::GtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a >= b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Lt => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a < b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::LtEq => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Boolean(a <= b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only compare numbers".into(),
                                line,
                            ))
                        }
                    }

                    BinOp::Add => {
                        if let (Value::Number(a), Value::Number(b)) = (&left_val, &right_val) {
                            Ok(Value::Number(a + b))
                        } else if let (Value::String(a), Value::String(b)) = (left_val, right_val) {
                            Ok(Value::String(format!("{a}{b}")))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only add numbers or strings".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Sub => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a - b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only subtract numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a / b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only divide numbers".into(),
                                line,
                            ))
                        }
                    }
                    BinOp::Mult => {
                        if let (Value::Number(a), Value::Number(b)) = (left_val, right_val) {
                            Ok(Value::Number(a * b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "can only multiply numbers".into(),
                                line,
                            ))
                        }
                    }
                }
            }

            Self::Unary(op, e) => {
                let val = e.eval(state)?;
                match op {
                    UnaryOp::Negative => {
                        if let Value::Number(n) = val {
                            Ok(Value::Number(-n))
                        } else {
                            Err(ErrorState::runtime_error(
                                "- can only be applied to numbers".into(),
                                line,
                            ))
                        }
                    }
                    UnaryOp::Inverse => {
                        if let Value::Boolean(b) = val {
                            Ok(Value::Boolean(!b))
                        } else {
                            Err(ErrorState::runtime_error(
                                "! can only be applied to numbers".into(),
                                line,
                            ))
                        }
                    }
                }
            }

            Self::Identifier(id) => Ok(state.get(id).unwrap_or(Value::Nil)),
            Self::StringLiteral(s) => Ok(Value::String(s.clone())),
            Self::NumberLiteral(n) => Ok(Value::Number(*n)),
            Self::True => Ok(Value::Boolean(true)),
            Self::False => Ok(Value::Boolean(false)),
            Self::Nil => Ok(Value::Nil),
        }
    }
}
