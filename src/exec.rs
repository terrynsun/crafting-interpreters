use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::HashMap;
use std::rc::Rc;

use crate::callable::{LoxFn, NativeFn};
use crate::config::Config;
use crate::error::ErrorState;
use crate::eval::Value;
use crate::grammar::{Decl, Program, Stmt};

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
        let mut global = Scope::new();

        let f = |_: &mut ExecState, _| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            Ok(Value::Number(now as f64))
        };
        global.insert(
            "clock".to_string(),
            Value::NativeFn(NativeFn::new(vec![], Rc::new(f))),
        );

        Self {
            scopes: vec![global],
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
    pub env: Environment,

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

    pub fn exec_decl(&mut self, decl: &Decl) -> Result<(), ErrorState> {
        match decl {
            Decl::VarDecl(id, expr) => {
                let val = expr.eval(self)?;
                self.env.insert(id.clone(), val);
            }

            Decl::FunDecl(name, parameters, body) => {
                let f = LoxFn::new(parameters.clone(), body.clone());
                self.env.insert(name.to_string(), Value::LoxFn(f));
            }

            Decl::Stmt(stmt) => self.eval_stmt(stmt)?,
        }

        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), ErrorState> {
        match stmt {
            Stmt::Expr(e) => {
                self.value = e.eval(self)?;
            }
            Stmt::Print(e) => {
                let val = e.eval(self)?;

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
                let condition = condition_expr.eval(self)?;
                if condition.is_truthy() {
                    self.eval_stmt(then_stmt)?;
                } else {
                    if let Some(else_stmt) = else_stmt {
                        self.eval_stmt(else_stmt)?;
                    }
                }
            }
            Stmt::While(condition_expr, body) => loop {
                let condition = condition_expr.eval(self)?;
                if condition.is_truthy() {
                    self.eval_stmt(body)?;
                } else {
                    break;
                }
            },
        }

        Ok(())
    }
}
