use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr};
use crate::env::Env;
use crate::error::TypeError;
use crate::values::Val;

use super::Type;

// `typ::check::er()` - get it?
pub fn er() -> TypeChecker {
    TypeChecker::new()
}

pub struct TypeChecker {
    typing_env: Env<Type>,
    next_unif_var: usize,
    unif_table: HashMap<usize, Type>,
}

impl TypeChecker {
    fn new() -> Self {
        let typing_env = Env::new();
        let next_unif_var = 0;
        let unif_table = Default::default();
        TypeChecker {
            typing_env,
            next_unif_var,
            unif_table,
        }
    }

    pub(crate) fn infer(&mut self, e: &Expr) -> Result<Type, TypeError> {
        use Expr::*;

        match e {
            Ascribed(e, t) => {
                println!("{e:?} is ascribed type {t:?}");
                let e_ty = self.infer(e.as_ref())?;
                println!("Inferred it to have type {e_ty:?}");
                self.unify(e_ty, t.clone())
            }
            App(fnc, args) => {
                // `fn_ty` is the inferred type of the function itself, based
                // on its body.
                // e.g. fn_ty = A_1 -> ... -> A_n
                let fn_ty = self.infer(fnc)?;
                // `arg_ty` is the inferred type of `fnc` based on the types of
                // the arguments we can see.
                // e.g. arg_ty = A_1 -> ... -> A_(n - m) -> UVar
                let arg_ty = args
                    .iter()
                    .rev()
                    .try_fold(self.new_unif_var(), |acc, nxt| {
                        let nxt_ty = self.infer(nxt)?;
                        Ok(Type::Arrow(Box::new(nxt_ty), Box::new(acc)))
                    })?;
                let resolved_ty = self.unify(fn_ty, arg_ty)?;

                resolved_ty.applied_to_args(args.len())
            }
            Lambda(tys, body) => {
                for t in tys {
                    self.typing_env.bind(t.clone());
                }
                let ret_ty = self.infer(body)?;
                for _ in 0..tys.len() {
                    self.typing_env.unbind();
                }

                let tys = tys.iter().rev().fold(ret_ty, |acc, nxt| {
                    Type::Arrow(Box::new(nxt.clone()), Box::new(acc))
                });

                Ok(tys)
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
                if cond_ty != Type::Bool {
                    Err(TypeError::Mismatch {
                        got: Type::Bool,
                        expected: cond_ty,
                    })
                } else if thn_ty != els_ty {
                    Err(TypeError::Mismatch {
                        got: thn_ty,
                        expected: els_ty,
                    })
                } else {
                    Ok(thn_ty)
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
                    }
                    _ => {
                        self.unify(l_ty, Type::Num)?;
                        self.unify(r_ty, Type::Num)?;
                        Ok(Type::Num)
                    }
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
            }
            (Type::UnificationVar(n), Type::UnificationVar(m))
                if !self.unif_table.contains_key(&n) && !self.unif_table.contains_key(&m) =>
            {
                Ok(Type::UnificationVar(n))
            }
            (t, Type::UnificationVar(n)) | (Type::UnificationVar(n), t) => {
                match self.unif_table.get(&n) {
                    None => {
                        self.unif_table.insert(n, t.clone());
                        Ok(t)
                    }
                    Some(t1) => self.unify(t, t1.clone()),
                }
            }
            (t1, t2) => Err(TypeError::Mismatch {
                got: t1,
                expected: t2,
            }),
        }
    }
}
