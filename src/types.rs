use std::collections::HashMap;
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
    UnificationVar(usize),
}

pub(crate) struct TypeChecker {
    typing_env: Env<Type>,
    next_unif_var: usize,
    unif_table: HashMap<usize, Type>,
}

impl TypeChecker {
    pub(crate) fn new() -> Self {
        let typing_env = Env::new();
        let next_unif_var = 0;
        let unif_table = Default::default();
        TypeChecker { typing_env, next_unif_var, unif_table }
    }

    pub(crate) fn infer(&mut self, e: &Expr) -> Result<Type, TypeError> {
        use Expr::*;

        match e {
            App(fnc, a) => {
                let fn_ty = self.infer(fnc)?;
                let a_ty = self.infer(a)?;

                let in_var = self.new_unif_var();
                let out_var = self.new_unif_var();
                let fn_ty = self.unify(fn_ty, Type::Arrow(Box::new(in_var), Box::new(out_var)))?;

                match fn_ty {
                    Type::Arrow(in_ty, out_ty) => {
                        self.unify(*in_ty, a_ty)?;
                        Ok(*out_ty)
                    }
                    _ => {
                        Err(TypeError::Mismatch)
                    }
                }
            }
            Lambda(Some(ty), body) => {
                self.typing_env.bind(ty.clone());
                let ret_ty = self.infer(body)?;
                self.typing_env.unbind();
                Ok(Type::Arrow(Box::new(ty.clone()), Box::new(ret_ty)))
            }
            Lambda(None, body) => {
                let unif_var = self.new_unif_var();
                self.typing_env.bind(unif_var);
                self.infer(body)
            }
            Let(false, binding, body) => {
                let binding_ty = self.infer(binding)?;
                self.typing_env.bind(binding_ty);
                self.infer(body)
            }
            Let(true, _, body) => {
                let unif_var = self.new_unif_var();
                self.typing_env.bind(unif_var);
                self.infer(body)
            }
            Literal(Val::Bool(_)) => Ok(Type::Bool),
            Literal(Val::Num(_)) => Ok(Type::Num),
            Literal(Val::Unit) => Ok(Type::Unit),
            Literal(Val::Closure { .. }) => unreachable!("We don't have closure literals."),
            Literal(Val::Dummy) => unreachable!("We don't have dummy literals"),
            IfThenElse(cond, thn, els) => {
                let cond_ty = self.infer(cond)?;
                let thn_ty = self.infer(thn)?;
                let els_ty = self.infer(els)?;
                if cond_ty == Type::Bool && thn_ty == els_ty {
                    Ok(thn_ty)
                } else {
                    Err(TypeError::Mismatch)
                }
            }
            Op(l, op, r) => {
                let l_ty = self.infer(l)?;
                let r_ty = self.infer(r)?;
                match op {
                    BinaryOp::Eq => Ok(Type::Bool),
                    BinaryOp::And => {
                        self.unify(l_ty, Type::Bool)?;
                        self.unify(r_ty, Type::Bool)?;
                        Ok(Type::Bool)
                    },
                    _ => {
                        self.unify(l_ty, Type::Num)?;
                        self.unify(r_ty, Type::Num)?;
                        Ok(Type::Num)
                    },
                }
            }
            Var(i) => Ok(self
                .typing_env
                .lookup(*i)
                .expect("Scope check should happen before typechecking")),
        }
    }

    fn new_unif_var(&mut self) -> Type {
        let t = Type::UnificationVar(self.next_unif_var);
        self.next_unif_var += 1;
        t
    }

    fn unify(&mut self, ty1: Type, ty2: Type) -> Result<Type, TypeError> {
        match (ty1, ty2) {
            (ty1, ty2) if ty1 == ty2 => Ok(ty1),
            (Type::Arrow(from1, to1), Type::Arrow(from2, to2)) => {
                let from = self.unify(*from1, *from2)?;
                let to = self.unify(*to1, *to2)?;
                Ok(Type::Arrow(Box::new(from), Box::new(to)))
            },
            (Type::UnificationVar(n), Type::UnificationVar(m)) if !self.unif_table.contains_key(&n) && !self.unif_table.contains_key(&m) => Ok(Type::UnificationVar(n)),
            (t, Type::UnificationVar(n)) | (Type::UnificationVar(n), t) => {
                match self.unif_table.get(&n) {
                    None => {
                        self.unif_table.insert(n, t.clone());
                        Ok(t)
                    }
                    Some(t1) => self.unify(t, t1.clone()),
                }
            },
            _ => {
                Err(TypeError::Mismatch)
            }
        }
    }
}
