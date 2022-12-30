use std::fmt::Debug;

use crate::ast::{BinaryOp, Expr};
use crate::env::Env;
use crate::error::TypeError;
use crate::values::Val;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    Bool,
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
            App(fnc, a) => {
                let fn_ty = self.check(fnc)?;
                let a_ty = self.check(a)?;

                match fn_ty {
                    Type::Arrow(in_ty, out_ty) if *in_ty == a_ty => Ok(*out_ty),
                    _ => Err(TypeError::Mismatch),
                }
            }
            Lambda(ty, body) => {
                self.typing_env.bind(ty.clone());
                let ret_ty = self.check(body)?;
                self.typing_env.unbind();
                Ok(Type::Arrow(Box::new(ty.clone()), Box::new(ret_ty)))
            }
            Let(binding, body) => {
                let binding_ty = self.check(binding)?;
                self.typing_env.bind(binding_ty);
                self.check(body)
            }
            Literal(Val::Bool(_)) => Ok(Type::Bool),
            Literal(Val::Num(_)) => Ok(Type::Num),
            Literal(Val::Unit) => Ok(Type::Unit),
            Literal(Val::Closure { .. }) => unreachable!("We don't have closure literals."),
            IfThenElse(cond, thn, els) => {
                let cond_ty = self.check(cond)?;
                let thn_ty = self.check(thn)?;
                let els_ty = self.check(els)?;
                if cond_ty == Type::Bool && thn_ty == els_ty {
                    Ok(thn_ty)
                } else {
                    Err(TypeError::Mismatch)
                }
            }
            Op(l, op, r) => {
                let l_ty = self.check(l)?;
                let r_ty = self.check(r)?;
                match (op, l_ty, r_ty) {
                    (BinaryOp::Eq, _, _) => Ok(Type::Bool),
                    (BinaryOp::And, Type::Bool, Type::Bool) => Ok(Type::Bool),
                    (_, Type::Num, Type::Num) => Ok(Type::Num),
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
