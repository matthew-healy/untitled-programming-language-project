use untitled_programming_language_project::{
    ast::{Expr, Opcode},
    check_types, evaluate, parse,
    types::Type,
    values::Val,
};

pub fn parse_successfully<'input>(input: &'input str) -> Expr {
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
pub fn mk_op<L, R>(l: L, op: Opcode, r: R) -> Expr
where
    L: Into<Expr>,
    R: Into<Expr>,
{
    Expr::Op(Box::new(l.into()), op, Box::new(r.into()))
}
