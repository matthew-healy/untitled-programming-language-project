use std::fmt::Debug;

use crate::ast::Expr;
use crate::error::TypeError;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Num,
}

pub(crate) struct TypeChecker;

impl TypeChecker {
    pub(crate) fn new() -> Self {
        TypeChecker
    }

    pub(crate) fn check(&self, e: Expr) -> Result<Type, TypeError> {
        use Expr::*;

        match e {
            Number(_) => Ok(Type::Num),
            Op(l, _, r) => {
                let l_ty = self.check(*l)?;
                let r_ty = self.check(*r)?;
                match (l_ty, r_ty) {
                    (Type::Num, Type::Num) => Ok(Type::Num),
                    _ => Err(TypeError::Mismatch),
                }
            }
        }
    }
}
