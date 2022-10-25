use std::collections::HashMap;
use std::fmt::Debug;

use crate::ast::{Expr, Ident};
use crate::error::TypeError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Num,
    Unit,
}

pub(crate) struct TypeChecker {
    typing_env: HashMap<Ident, Type>,
}

impl TypeChecker {
    pub(crate) fn new() -> Self {
        let typing_env = HashMap::new();
        TypeChecker { typing_env }
    }

    pub(crate) fn check(&mut self, e: &Expr) -> Result<Type, TypeError> {
        use Expr::*;

        match e {
            Let(i, binding, body) => {
                let binding_ty = self.check(binding)?;
                self.typing_env.insert(i.clone(), binding_ty);
                self.check(body)
            }
            Number(_) => Ok(Type::Num),
            Op(l, _, r) => {
                let l_ty = self.check(l)?;
                let r_ty = self.check(r)?;
                match (l_ty, r_ty) {
                    (Type::Num, Type::Num) => Ok(Type::Num),
                    _ => Err(TypeError::Mismatch),
                }
            }
            Unit => Ok(Type::Unit),
            Var(i) => self
                .typing_env
                .get(i)
                .ok_or(TypeError::UnboundIdent(i.clone()))
                .map(|t| t.clone()),
        }
    }
}
