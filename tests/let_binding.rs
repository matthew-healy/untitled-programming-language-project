use untitled_programming_language_project::{
    ast, check_types,
    error::{Error, TypeError},
    parse,
    types::Type,
    values::Val,
};

pub mod common;
use common::{evaluate_successfully, mk_op, parse_successfully};

use crate::common::typecheck_successfully;

fn mk_let(i: ast::Ident, v: ast::Expr, body: ast::Expr) -> ast::Expr {
    ast::Expr::Let(i, Box::new(v), Box::new(body))
}

#[test]
fn parsing() {
    use ast::*;

    for (name, input, expected) in [
        (
            "simple Num",
            "let x = 1 in x",
            mk_let("x".into(), Expr::Number(1), Expr::Var("x".into())),
        ),
        (
            "unused binding",
            "let hello = () in ()",
            mk_let("hello".into(), Expr::Unit, Expr::Unit),
        ),
        (
            "nested",
            "let x = 1 in let y = 2 in x + y",
            mk_let(
                "x".into(),
                Expr::Number(1),
                mk_let(
                    "y".into(),
                    Expr::Number(2),
                    mk_op(Expr::Var("x".into()), Opcode::Add, Expr::Var("y".into())),
                ),
            ),
        ),
        (
            "ident starting with underscore",
            "let _a = 1 in _a",
            mk_let("_a".into(), Expr::Number(1), Expr::Var("_a".into())),
        ),
        (
            "ident with numbers",
            "let n0 = 100 in n0",
            mk_let("n0".into(), Expr::Number(100), Expr::Var("n0".into())),
        ),
    ] {
        let actual = parse_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

#[test]
fn invalid_identifiers() {
    for invalid_ident in ["!no", "99invalid", "___", "-buuuu", "?que", "$nah"] {
        let expr = format!("let {} = 1 in {}", invalid_ident, invalid_ident);
        let result = parse(&expr);
        assert!(result.is_err())
    }
}

#[test]
fn typecheck() {
    for (name, input, expected) in [
        ("num binding", "let x = 0 in x", Type::Num),
        ("unit binding", "let no = () in no", Type::Unit),
        (
            "unused binding doesn't impact type",
            "let unused = 55 in ()",
            Type::Unit,
        ),
        (
            "shadowing with a different type",
            "let one = () in let one = 1 in one",
            Type::Num
        )
    ] {
        let actual = typecheck_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

#[test]
fn evaluation() {
    for (name, input, expected) in [
        ("num", "let x = -110 in x", Val::Num(-110)),
        ("unit", "let eep = () in eep", Val::Unit),
        (
            "unused binding doesn't impact value",
            "let unused = () in 45",
            Val::Num(45),
        ),
        (
            "nested bindings",
            "let a = -100 in let b = 100 in a + b",
            Val::Num(0),
        ),
        (
            "name shadowing",
            "let a = 10 in let a = 20 in a",
            Val::Num(20),
        )
    ] {
        let actual = evaluate_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

#[test]
fn recursive_let_is_not_supported() {
    let result = check_types("let x = x in x");
    let expected = Error::TypeError(TypeError::UnboundIdent("x".into()));
    assert_eq!(Err(expected), result)
}
