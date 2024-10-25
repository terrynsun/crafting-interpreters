use std::collections::HashMap;

use crate::config::Config;
use crate::error::ErrorState;
use crate::eval::Value;
use crate::grammar::{Decl, ExprData, Program, Stmt};

/// Simple wrapper around one scope.
pub struct Scope {
    data: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn contains(&self, k: &String) -> bool {
        self.data.contains_key(k)
    }

    pub fn insert(&mut self, k: String, v: Value) {
        self.data.insert(k, v);
    }

    pub fn get(&self, k: &String) -> Option<Value> {
        self.data.get(k).cloned()
    }
}

/// A stack of nested scopes.
pub struct Environment {
    scopes: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn open_scope(&mut self) {
        self.scopes.push(Scope::new())
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            panic!("cannot remove global scope");
        }
    }

    pub fn contains(&self, k: &String) -> bool {
        self.scopes.iter().any(|s| s.contains(k))
    }

    /// Insert `k` into the topmost scope. Used for declarations.
    pub fn insert(&mut self, k: String, v: Value) {
        self.scopes.last_mut().unwrap().insert(k, v);
    }

    /// Update the last scope that contains `k`. Used for assignments.
    pub fn update(&mut self, k: String, v: Value) -> Result<(), ()> {
        if let Some(scope) = self.scopes.iter_mut().rev().find(|s| s.contains(&k)) {
            scope.insert(k, v);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self, k: &String) -> Option<Value> {
        self.scopes.iter().rev().find_map(|s| s.get(&k))
    }
}

pub struct ExecState {
    config: Config,

    /// Variable store.
    env: Environment,

    /// If this value is populated, write print statements into this string so they can be captured
    /// for tests.
    print_target: Option<String>,

    /// The last value that was evaluated. Stored here so the repl can print it.
    pub value: Value,
}

impl ExecState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            env: Environment::new(),
            print_target: None,
            value: Value::Nil,
        }
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        Self {
            config: Config {
                file: None,
                debug_ast: false,
            },
            env: Environment::new(),
            print_target: Some(String::new()),
            value: Value::Nil,
        }
    }

    #[cfg(test)]
    pub fn assert_output(&self, expected: &str) {
        if let Some(s) = &self.print_target {
            assert_eq!(*s, expected);
        }
    }

    fn print(&mut self, output: &str) {
        if let Some(s) = &mut self.print_target {
            s.push_str(output);
            s.push_str("\n");
        } else {
            println!("{}", output);
        }
    }

    pub fn exec(&mut self, program: &Program) -> Result<(), ErrorState> {
        self.value = Value::Nil;

        for decl in program {
            if self.config.debug_ast {
                decl.pretty();
            }

            self.exec_decl(decl)?;
        }

        Ok(())
    }

    fn exec_decl(&mut self, decl: &Decl) -> Result<(), ErrorState> {
        match decl {
            Decl::VarDecl(id, expr) => {
                let val = expr.eval(&mut self.env)?;

                match &id.data {
                    ExprData::Identifier(s) => {
                        self.env.insert(s.clone(), val);
                    }
                    _ => {
                        // I think this should have been checked during parsing, which is why it's
                        // a panic.
                        panic!("expected identifier");
                    }
                }
            }
            Decl::Stmt(stmt) => self.eval_stmt(stmt)?,
        }

        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), ErrorState> {
        match stmt {
            Stmt::Expr(e) => {
                self.value = e.eval(&mut self.env)?;
            }
            Stmt::Print(e) => {
                let val = e.eval(&mut self.env)?;

                self.print(&val.to_string());
            }
            Stmt::Block(decls) => {
                self.env.open_scope();
                for d in decls {
                    self.exec_decl(d)?;
                }
                self.env.pop_scope();
            }
            Stmt::If(condition_expr, then_stmt, else_stmt) => {
                let condition = condition_expr.eval(&mut self.env)?;
                if condition.is_truthy() {
                    self.eval_stmt(then_stmt)?;
                } else {
                    if let Some(else_stmt) = else_stmt {
                        self.eval_stmt(else_stmt)?;
                    }
                }
            }
            Stmt::While(condition_expr, body) => {
                loop {
                    let condition = condition_expr.eval(&mut self.env)?;
                    if condition.is_truthy() {
                        self.eval_stmt(body)?;
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
