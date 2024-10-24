use std::collections::HashMap;

use crate::config::Config;
use crate::error::ErrorState;
use crate::eval::Value;
use crate::expr::{Decl, ExprData, Program, Stmt};

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

    // Insert `k` into the topmost scope. Used for declarations.
    pub fn insert(&mut self, k: String, v: Value) {
        self.scopes.last_mut().unwrap().insert(k, v);
    }

    // Update the last scope that contains `k`. Used for assignments.
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
    env: Environment,
}

impl ExecState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            env: Environment::new(),
        }
    }

    pub fn exec(&mut self, program: Program) -> Result<(), ErrorState> {
        for decl in program {
            if self.config.debug_ast {
                decl.pretty();
            }

            self.exec_decl(decl)?;
        }

        Ok(())
    }

    fn exec_decl(&mut self, decl: Decl) -> Result<(), ErrorState> {
        match decl {
            Decl::VarDecl(id, expr) => {
                let val = expr.eval(&mut self.env)?;

                match id.data {
                    ExprData::Identifier(s) => {
                        self.env.insert(s, val);
                    }
                    _ => {
                        panic!("expected identifier");
                    }
                }
            }
            Decl::Stmt(stmt) => match stmt {
                Stmt::Expr(e) => {
                    let val = e.eval(&mut self.env);
                    match val {
                        Ok(_v) => (),
                        Err(e) => println!("{e}"),
                    }
                }
                Stmt::Print(e) => {
                    let val = e.eval(&mut self.env);
                    match val {
                        Ok(v) => println!("{v}"),
                        Err(e) => println!("{e}"),
                    }
                }
                Stmt::Block(decls) => {
                    self.env.open_scope();
                    for d in decls {
                        self.exec_decl(d)?;
                    }
                    self.env.pop_scope();
                }
            },
        }

        Ok(())
    }
}
