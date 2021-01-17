use std::collections::HashMap;

use crate::{
    block::Block,
    eval::{atom::Atom, eval_expr},
    parser::*,
};

#[derive(Debug)]
pub struct State {
    pub scopes: Vec<Scope>,
}

impl Default for State {
    fn default() -> Self {
        State {
            scopes: vec![Scope::default()],
        }
    }
}

impl State {
    pub fn get_variable(&self, var: &String) -> Option<&Atom> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.vars.get(var))
    }

    fn modify_variable(&mut self, var: &String, val: Atom) {
        for scope in &mut self.scopes {
            if scope.vars.contains_key(var) {
                scope.vars.insert(var.to_string(), val);
                break;
            }
        }
    }

    pub fn declare(&mut self, dec: Declaration) {
        let disc = {
            let var = self.get_variable(&dec.lhs);
            var.map(std::mem::discriminant)
        };

        match (disc, dec.alias) {
            (Some(d), alias) => {
                let new_val = eval_expr(&dec.rhs, self);
                if d == std::mem::discriminant(&&new_val) || alias {
                    self.modify_variable(&dec.lhs, new_val);
                } else {
                    panic!(
                        "Mismatched types for {}, can't assign {:?} to {:?}",
                        dec.lhs,
                        new_val,
                        self.get_variable(&dec.lhs).unwrap()
                    );
                }
            }
            (None, true) => {
                let new_val = eval_expr(&dec.rhs, self);
                self.scopes
                    .last_mut()
                    .unwrap()
                    .vars
                    .insert(dec.lhs, new_val);
            }
            (None, false) => {
                panic!("Uninitialized variable {}", dec.lhs)
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Scope {
    pub vars: HashMap<String, Atom>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub lhs: String,
    pub rhs: S,
    pub alias: bool,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: S,
    pub then_block: Block,
    pub else_block: Block,
}

#[derive(Debug, Clone)]
pub struct While {
    pub cond: S,
    pub loop_block: Block,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(S),
    PrintStmt(S),
    Dec(Declaration),
    IfStmt(If),
    WhileStmt(While),
}

impl Stmt {
    pub fn execute(self, state: &mut State) -> Option<Atom> {
        match self {
            Stmt::ExprStmt(expr) => Some(eval_expr(&expr, state)),
            Stmt::PrintStmt(expr) => {
                println!("{}", eval_expr(&expr, state));
                None
            }
            Stmt::Dec(dec) => {
                state.declare(dec);
                None
            }
            Stmt::IfStmt(if_data) => {
                let If {
                    cond,
                    mut then_block,
                    mut else_block,
                } = if_data;

                if eval_expr(&cond, state) == Atom::Bool(true) {
                    then_block.execute(state)
                } else {
                    else_block.execute(state)
                }
            }
            Stmt::WhileStmt(while_data) => {
                let While {
                    cond,
                    mut loop_block,
                } = while_data;

                while eval_expr(&cond, state) == Atom::Bool(true) {
                    loop_block.execute(state);
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use crate::run_file;
    use crate::Atom;
    use crate::State;

    macro_rules! test_files {
        () => {};
        ( $fn_name:ident, $file:expr => $expected:expr; $($tail:tt)* ) => {
            #[test]
            fn $fn_name() {
                let mut top_state = State::default();
                let output = run_file(format!("test_files/{}", $file), &mut top_state).unwrap();
                assert_eq!(output, $expected);
            }

            test_files!($($tail)*);
        };
        ( $fn_name:ident, $file:expr; $($tail:tt)* ) => {
            #[test]
            #[should_panic]
            fn $fn_name() {
                let mut top_state = State::default();
                run_file(format!("test_files/{}", $file), &mut top_state).unwrap();
            }

            test_files!($($tail)*);
        };
    }

    test_files!(
        basic1, "basic1.slang" => Some(Atom::Num(20.0));
        basic2, "basic2.slang" => Some(Atom::Num(5.0));
        if1, "if.slang" => Some(Atom::Str("hello".to_string()));
        if2, "else.slang" => Some(Atom::Str("goodbye".to_string()));
        scope_modify, "scope_modify.slang" => Some(Atom::Num(2.0));
        while1, "while1.slang" => Some(Atom::Num(10.0));
        error1, "error1.slang";
        scope_typecheck, "scope_typecheck.slang";
    );
}
