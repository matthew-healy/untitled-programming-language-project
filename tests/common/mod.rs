use untitled_programming_language_project::{
    ast::{Opcode, RawExpr},
    check_types, evaluate, parse,
    types::Type,
    values::Val,
};

pub fn parse_successfully<'input>(input: &'input str) -> RawExpr {
    *parse(input).unwrap_or_else(|e| {
        panic!(
            "unexpected parse failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

pub fn typecheck_successfully<'input>(input: &'input str) -> Type {
    check_types(input).unwrap_or_else(|e| {
        panic!(
            "unexpected typecheck failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

pub fn evaluate_successfully<'input>(input: &'input str) -> Val {
    evaluate(input).unwrap_or_else(|e| {
        panic!(
            "unexpected evaluation failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

/// make a binary op from two expressions.
pub fn mk_op<L, R>(l: L, op: Opcode, r: R) -> RawExpr
where
    L: Into<RawExpr>,
    R: Into<RawExpr>,
{
    RawExpr::Op(Box::new(l.into()), op, Box::new(r.into()))
}
