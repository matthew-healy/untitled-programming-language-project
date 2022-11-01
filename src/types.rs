use std::fmt::Debug;

use crate::ast::Expr;
use crate::env::Env;
use crate::error::TypeError;
use crate::values::Val;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Num,
    Unit,
}

pub(crate) struct TypeChecker {
    typing_env: Env<Type>,
}

impl TypeChecker {
    pub(crate) fn new() -> Self {
        let typing_env = Env::new();
        TypeChecker { typing_env }
    }

    pub(crate) fn check(&mut self, e: &Expr) -> Result<Type, TypeError> {
        use Expr::*;

        match e {
            Let(binding, body) => {
                let binding_ty = self.check(binding)?;
                self.typing_env.bind(binding_ty);
                self.check(body)
            }
            Literal(Val::Num(_)) => Ok(Type::Num),
            Literal(Val::Unit) => Ok(Type::Unit),
            Op(l, _, r) => {
                let l_ty = self.check(l)?;
                let r_ty = self.check(r)?;
                match (l_ty, r_ty) {
                    (Type::Num, Type::Num) => Ok(Type::Num),
                    _ => Err(TypeError::Mismatch),
                }
            }
            Var(i) => Ok(self
                .typing_env
                .lookup(*i)
                .expect("Scope check should happen before typechecking")
                .clone()),
        }
    }
}
