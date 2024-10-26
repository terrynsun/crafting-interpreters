use std::rc::Rc;

use crate::error::ErrorState;
use crate::eval::Value;
use crate::exec::ExecState;
use crate::grammar::Stmt;

pub trait Callable {
    fn call(&self, env: &mut ExecState, args: Vec<Value>) -> Result<Value, ErrorState>;
    fn arity(&self) -> u32;
}

#[derive(Clone)]
pub struct NativeFn {
    // array of identifiers
    parameters: Vec<String>,

    body: Rc<dyn Fn(&mut ExecState, Vec<Value>) -> Result<Value, ErrorState>>,
}

impl NativeFn {
    pub fn new(
        parameters: Vec<String>,
        body: Rc<dyn Fn(&mut ExecState, Vec<Value>) -> Result<Value, ErrorState>>,
    ) -> Self {
        Self { parameters, body }
    }
}

impl Callable for NativeFn {
    fn arity(&self) -> u32 {
        self.parameters.len() as u32
    }

    fn call(&self, state: &mut ExecState, args: Vec<Value>) -> Result<Value, ErrorState> {
        for (arg, param) in args.iter().zip(self.parameters.iter()) {
            state.env.insert(param.clone(), arg.clone());
        }

        (self.body)(state, args)
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
    parameters: Vec<String>,

    body: Stmt,
}

impl LoxFn {
    pub fn new(parameters: Vec<String>, body: Stmt) -> Self {
        Self { parameters, body }
    }
}

impl Callable for LoxFn {
    fn arity(&self) -> u32 {
        self.parameters.len() as u32
    }

    fn call(&self, state: &mut ExecState, args: Vec<Value>) -> Result<Value, ErrorState> {
        for (arg, param) in args.iter().zip(self.parameters.iter()) {
            state.env.insert(param.clone(), arg.clone());
        }

        if let Stmt::Block(decls) = &self.body {
            for d in decls {
                state.exec_decl(&d)?;
            }
            // todo: check for return statement
        }

        Ok(Value::Nil)
    }
}
