use std::fmt::Debug;

use crate::ast::Expr;

pub enum Error {
    Mismatch,
}

// TODO: Display
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        let desc = match self {
            Mismatch => "Type mismatch",
        };

        write!(f, "[Error] {}", desc)
    }
}

pub enum Type {
    Num,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;

        let desc = match self {
            Num => "Num",
        };
        write!(f, "{}", desc)
    }
}

pub struct TypeChecker;

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker
    }

    pub fn check(&self, e: Expr) -> Result<Type, Error> {
        use Expr::*;

        match e {
            Number(_) => Ok(Type::Num),
            Op(l, _, r) => {
                let l_ty = self.check(*l)?;
                let r_ty = self.check(*r)?;
                match (l_ty, r_ty) {
                    (Type::Num, Type::Num) => Ok(Type::Num),
                    _ => Err(Error::Mismatch),
                }
            }
        }
    }
}