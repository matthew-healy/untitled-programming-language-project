use crate::ast::{BinaryOp, Expr};

use crate::values::Val;

use super::ctx::Ctx;
use super::{ctx, Error, Existential, Primitive, Type};

pub(crate) struct State {
    next_existential: usize,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            next_existential: 0,
        }
    }

    /// Generate a fresh `Existential`, guaranteed to be distinct from all
    /// others.
    fn fresh_existential(&mut self) -> Existential {
        let e = Existential(self.next_existential);
        self.next_existential += 1;
        e
    }
}

/// Synthesize a type for `e`. Returns both the synthesized `Type` and the
/// updated `Ctx` if successful, and an `Error` otherwise.
pub(crate) fn synthesize_type(state: &mut State, ctx: Ctx, e: &Expr) -> Result<(Type, Ctx), Error> {
    match e {
        Expr::Literal(l) => Ok((l.typ(), ctx)),
        Expr::Var(id, _) => {
            let t = ctx.get_annotation(*id)?;
            Ok((t.clone(), ctx))
        }
        Expr::Ascribed(e, t) => {
            ctx.check_type_well_formed(t)?;
            let ctx = check_type(state, ctx, e, t)?;
            Ok((t.clone(), ctx))
        }
        // TODO: lambdas don't need types anymore. (though we likely want to
        // optionally include them & use them if we get them).
        Expr::Lambda(id, _, e) => {
            let from = state.fresh_existential();
            let to = state.fresh_existential();

            // Insert the existential types into the context, as well as `v: inferred_from`.
            let ctx = ctx
                .add(ctx::Element::Existential(from.clone()))
                .add(ctx::Element::Existential(to.clone()))
                .add(ctx::Element::TypedVariable(
                    id.clone(),
                    Type::Existential(from.clone()),
                ));
            // Then check that the lambda's body typechecks as `inferred_to` in that context
            let ctx = check_type(state, ctx, e, &Type::Existential(to.clone()))?;
            // if so then it must have type `inferred_from -> inferred_to`
            Ok((
                Type::Arrow(
                    Box::new(Type::Existential(from.clone())),
                    Box::new(Type::Existential(to.clone())),
                ),
                ctx,
            ))
        }
        Expr::App(e1, e2) => {
            let (t, ctx) = synthesize_type(state, ctx, e1)?;
            match t.apply(&ctx) {
                // α^App
                Type::Existential(a) => {
                    let from = state.fresh_existential();
                    let to = state.fresh_existential();

                    let ctx = ctx.insert_in_place(
                        ctx::Element::Existential(a.clone()),
                        &[
                            ctx::Element::Existential(to.clone()),
                            ctx::Element::Existential(from.clone()),
                            ctx::Element::Solved(
                                a,
                                Type::Arrow(
                                    Box::new(Type::Existential(from.clone())),
                                    Box::new(Type::Existential(to.clone())),
                                ),
                            ),
                        ],
                    )?;
                    let ctx = check_type(state, ctx, e2, &Type::Existential(from))?;
                    Ok((Type::Existential(to), ctx))
                }
                Type::Arrow(from, to) => {
                    let ctx = check_type(state, ctx, e2, &from)?;
                    Ok((*to, ctx))
                }
                t => Err(Error::InvalidApplication(t)),
            }
        }
        Expr::Let(false, id, binding, body) => {
            let (binding_type, ctx) = synthesize_type(state, ctx, binding)?;
            let ctx = ctx.add(ctx::Element::TypedVariable(*id, binding_type.clone()));

            let (body_type, ctx) = synthesize_type(state, ctx, body)?;
            Ok((
                body_type,
                ctx.insert_in_place(ctx::Element::TypedVariable(*id, binding_type), &[])?,
            ))
        }
        Expr::Let(true, id, binding, body) => {
            let binding_existential = state.fresh_existential();
            let ctx = ctx.add(ctx::Element::Existential(binding_existential)).add(
                ctx::Element::TypedVariable(*id, Type::Existential(binding_existential)),
            );

            let (_, ctx) = synthesize_type(state, ctx, binding)?;
            let (body_type, ctx) = synthesize_type(state, ctx, body)?;

            let ctx = ctx.insert_in_place(
                ctx::Element::TypedVariable(*id, Type::Existential(binding_existential)),
                &[],
            )?;

            Ok((body_type, ctx))
        }
        Expr::IfThenElse(cond, thn, els) => {
            let (cond_type, ctx) = synthesize_type(state, ctx, cond)?;
            let ctx = subtype(state, ctx, &cond_type, &Type::bool())?;
            let (thn_ty, ctx) = synthesize_type(state, ctx, thn)?;
            let ctx = check_type(state, ctx, els, &thn_ty)?;
            Ok((thn_ty, ctx))
        }
        Expr::Op(l, op, r) => {
            let (l_ty, ctx) = synthesize_type(state, ctx, l)?;
            let (r_ty, ctx) = synthesize_type(state, ctx, r)?;

            match op {
                BinaryOp::Eq => Ok((Type::bool(), ctx)),
                BinaryOp::And => {
                    let bl = Type::bool();
                    let l_ty = l_ty.apply(&ctx);
                    let ctx = subtype(state, ctx, &l_ty, &bl)?;
                    let r_ty = r_ty.apply(&ctx);
                    let ctx = subtype(state, ctx, &r_ty, &bl)?;
                    Ok((bl, ctx))
                }
                BinaryOp::Mul | BinaryOp::Div | BinaryOp::Add | BinaryOp::Sub => {
                    let num = Type::num();
                    let l_ty = l_ty.apply(&ctx);
                    let ctx = subtype(state, ctx, &l_ty, &num)?;
                    let r_ty = r_ty.apply(&ctx);
                    let ctx = subtype(state, ctx, &r_ty, &num)?;
                    Ok((num, ctx))
                }
            }
        }
    }
}

/// Check that `e` has type `t`. Returns an updated `Ctx` if it is, and an
/// `Error` otherwise.
fn check_type(state: &mut State, ctx: Ctx, e: &Expr, t: &Type) -> Result<Ctx, Error> {
    ctx.check_type_well_formed(t)?;

    match (e, t) {
        (Expr::Literal(l), Type::Primitive(p)) => check_literal_type(ctx, l, p),
        (Expr::Lambda(id, _, e), Type::Arrow(from_ty, to_ty)) => {
            // Insert the newly known type - i.e. `v: from_ty` - into the context
            let typed_var = ctx::Element::TypedVariable(*id, *from_ty.clone());
            let ctx = ctx.add(typed_var.clone());
            // Check the body of the lambda against the `to_ty`, and then, if we succeed,
            // drop everything from `typed_var` on from the context.
            // Note that in particular any earlier existentials which got solved during this
            // call will stick around.
            check_type(state, ctx, e, to_ty)?.drop(&typed_var)
        }
        (_, _) => {
            let (inferred_t, ctx) = synthesize_type(state, ctx, e)?;
            let a = inferred_t.apply(&ctx);
            let t = t.clone().apply(&ctx);
            subtype(state, ctx, &a, &t)
        }
    }
}

/// Ensures that `a` is a subtype of `b`. Returns an updated `Ctx` if it is, and
/// an `Error` otherwise.
fn subtype(state: &mut State, ctx: Ctx, a: &Type, b: &Type) -> Result<Ctx, Error> {
    ctx.check_type_well_formed(a)?;
    ctx.check_type_well_formed(b)?;

    match (a, b) {
        (Type::Primitive(p1), Type::Primitive(p2)) if p1 == p2 => Ok(ctx),
        (Type::Existential(e1), Type::Existential(e2)) if e1 == e2 => Ok(ctx),
        (Type::Arrow(from1, to1), Type::Arrow(from2, to2)) => {
            let ctx = subtype(state, ctx, from2, from1)?;
            let to1 = to1.apply(&ctx);
            let to2 = to2.apply(&ctx);
            let r = subtype(state, ctx, &to1, &to2)?;
            Ok(r)
        }
        (Type::Existential(to_instantiate), _) => instantiate_l(state, ctx, *to_instantiate, b),
        (_, Type::Existential(to_instantiate)) => instantiate_r(state, ctx, a, *to_instantiate),
        (_, _) => Err(Error::Mismatch {
            got: a.clone(),
            expected: b.clone(),
        }),
    }
}

/// Instantiate `to_instantiate` with `t` as an upper-bound.
fn instantiate_l(
    state: &mut State,
    ctx: Ctx,
    to_instantiate: Existential,
    t: &Type,
) -> Result<Ctx, Error> {
    let (left, right) = ctx.split_at(&ctx::Element::Existential(to_instantiate.to_owned()))?;

    if left.check_type_well_formed(t).is_ok() {
        ctx.insert_in_place(
            ctx::Element::Existential(to_instantiate.to_owned()),
            &[ctx::Element::Solved(to_instantiate.to_owned(), t.clone())],
        )
    } else {
        match t {
            Type::Arrow(from, to) => {
                // Instantiate `to_instantiate` to an arrow `e1 -> e2` and then
                // check that `e1 -> e2 <: from -> to` (i.e. that `from <: e1`
                // and `e2 <: to`)
                let inferred_from = state.fresh_existential();
                let inferred_to = state.fresh_existential();

                let ctx = ctx.insert_in_place(
                    ctx::Element::Existential(to_instantiate.to_owned()),
                    &[
                        ctx::Element::Existential(inferred_to.clone()),
                        ctx::Element::Existential(inferred_from.clone()),
                        ctx::Element::Solved(
                            to_instantiate.to_owned(),
                            Type::Arrow(
                                Box::new(Type::Existential(inferred_from.clone())),
                                Box::new(Type::Existential(inferred_to.clone())),
                            ),
                        ),
                    ],
                )?;

                let ctx = instantiate_r(state, ctx, from, inferred_from)?;
                let to = to.apply(&ctx);
                instantiate_l(state, ctx, inferred_to, &to)
            }
            Type::Existential(e) => {
                right.check_type_well_formed(t)?;
                ctx.insert_in_place(
                    ctx::Element::Existential(e.clone()),
                    &[ctx::Element::Solved(
                        e.clone(),
                        Type::Existential(to_instantiate.to_owned()),
                    )],
                )
            }
            Type::Primitive(_) => unreachable!("handled in first branch of if"),
        }
    }
}

/// Instantiates `to_instantiate` with `t` as a lower-bound.
fn instantiate_r(
    state: &mut State,
    ctx: Ctx,
    t: &Type,
    to_instantiate: Existential,
) -> Result<Ctx, Error> {
    let (left, right) = ctx.split_at(&ctx::Element::Existential(to_instantiate))?;

    if left.check_type_well_formed(t).is_ok() {
        ctx.insert_in_place(
            ctx::Element::Existential(to_instantiate.into()),
            &[ctx::Element::Solved(to_instantiate.into(), t.clone())],
        )
    } else {
        match t {
            Type::Arrow(from, to) => {
                // Instantiate `to_instantiate` to an arrow `e1 -> e2` and then
                // check that `from -> to <: e1 -> e2` (i.e. that `e1 <: from`
                // and `to <: e2`)
                let inferred_from = state.fresh_existential();
                let inferred_to = state.fresh_existential();

                let ctx = ctx
                    .add(ctx::Element::Existential(inferred_to.clone()))
                    .add(ctx::Element::Existential(inferred_from.clone()))
                    .add(ctx::Element::Solved(
                        to_instantiate.into(),
                        Type::Arrow(
                            Box::new(Type::Existential(inferred_from.clone())),
                            Box::new(Type::Existential(inferred_to.clone())),
                        ),
                    ));

                let ctx = instantiate_l(state, ctx, inferred_from, from)?;
                let to = to.apply(&ctx);
                instantiate_r(state, ctx, &to, inferred_to)
            }
            Type::Existential(e) => {
                right.check_type_well_formed(t)?;
                Ok(ctx.add(ctx::Element::Solved(
                    e.clone(),
                    Type::Existential(to_instantiate.into()),
                )))
            }
            Type::Primitive(_) => unreachable!("handled in first branch of if"),
        }
    }
}

/// Check whether value `l` has primitive type `p`.
fn check_literal_type(ctx: Ctx, l: &Val, p: &Primitive) -> Result<Ctx, Error> {
    match (l, p) {
        (Val::Bool(_), Primitive::Bool)
        | (Val::Num(_), Primitive::Num)
        | (Val::Unit, Primitive::Unit) => Ok(ctx),
        (Val::Closure { .. } | Val::Dummy, _) => unreachable!("Runtime-only"),
        (l, p) => Err(Error::Mismatch {
            got: l.typ(),
            expected: Type::Primitive(p.clone()),
        }),
    }
}

impl Type {
    /// Apply a `Ctx` to the `Type`, replacing any solved existentials with
    /// their solutions.
    pub(crate) fn apply(&self, ctx: &Ctx) -> Self {
        match self {
            Type::Arrow(from, to) => {
                Type::Arrow(Box::new(from.apply(ctx)), Box::new(to.apply(ctx)))
            }
            Type::Existential(a) => match ctx.get_solved(&a) {
                Some(t) => t.clone().apply(ctx),
                None => self.clone(),
            },
            Type::Primitive(p) => Type::Primitive(*p),
        }
    }
}

impl Val {
    /// Get the primitive type of `self`. Panics if called on a `Closure` or
    /// `Dummy` value.
    fn typ(&self) -> Type {
        Type::Primitive(match self {
            Val::Bool(_) => Primitive::Bool,
            Val::Num(_) => Primitive::Num,
            Val::Unit => Primitive::Unit,
            Val::Closure { .. } | Val::Dummy => unreachable!("Runtime-only"),
        })
    }
}
