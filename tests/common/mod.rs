use untitled_programming_language_project::{ast, check_types, parse, types::Type};

pub fn parse_successfully<'input>(input: &'input str) -> ast::Expr {
    parse(input).unwrap_or_else(|e| {
        panic!(
            "unexpected parse failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

pub fn typecheck_successfully<'input>(input: &'input str) -> Type {
    check_types(input)
        .unwrap_or_else(|e| panic!("unexpected failure.\ninput: {}\nerror: {:?}", input, e))
}

/// make a binary op from two expressions.
pub fn mk_op<L, R>(l: L, op: ast::Opcode, r: R) -> ast::Expr
where
    L: Into<ast::Expr>,
    R: Into<ast::Expr>,
{
    ast::Expr::Op(Box::new(l.into()), op, Box::new(r.into()))
}