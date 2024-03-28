use std::collections::HashMap;

use crate::config::Config;
use crate::error::ErrorState;
use crate::eval::Value;
use crate::expr::{Decl, ExprData, Program, Stmt};

pub struct Environment {
    globals: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: Value) {
        self.globals.insert(k, v);
    }

    pub fn get(&mut self, k: &String) -> Option<Value> {
        Some(self.globals[k].clone())
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
                },
            }
        }

        Ok(())
    }
}
