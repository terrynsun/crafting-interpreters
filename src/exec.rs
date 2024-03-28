use std::collections::HashMap;

use crate::config::Config;
use crate::error::ErrorState;
use crate::eval::Value;
use crate::expr::{Decl, Program, Stmt, ExprData};

pub struct State {
    config: Config,
    globals: HashMap<String, Value>,
}

impl State {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            globals: HashMap::new(),
        }
    }

    pub fn exec(&mut self, program: Program) -> Result<(), ErrorState> {
        for decl in program {
            match decl {
                Decl::VarDecl(id, expr) => {
                    let val = expr.eval()?;

                    match id.data {
                        ExprData::Identifier(s) => {
                            self.globals.insert(s, val);
                        }
                        _ => {
                            panic!("expected identifier");
                        }
                    }
                }
                Decl::Stmt(stmt) => match stmt {
                    Stmt::Expr(e) => {
                        if self.config.debug_ast {
                            e.pretty();
                        }

                        let val = e.eval();
                        match val {
                            Ok(_v) => (),
                            Err(e) => println!("{e}"),
                        }
                    }
                    Stmt::Print(e) => {
                        if self.config.debug_ast {
                            e.pretty();
                        }

                        let val = e.eval();
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
