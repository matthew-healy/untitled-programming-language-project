use untitled_programming_language_project::{
    ast, check_types,
    error::{Error, EvaluationError, TypeError},
    evaluate,
    types::Type,
    values::Val,
};

pub mod common;
use common::{mk_op, parse_successfully, typecheck_successfully};

#[test]
fn parsing() {
    use ast::*;

    for (name, input, expected) in [
        ("addition", "1 + 1", mk_op(1.0, BinaryOp::Add, 1.0)),
        ("subtraction", "99 - 4", mk_op(99.0, BinaryOp::Sub, 4.0)),
        (
            "multiplication",
            "-3 * -914",
            mk_op(-3.0, BinaryOp::Mul, -914.0),
        ),
        (
            "division",
            "4444 / 1111",
            mk_op(4444.0, BinaryOp::Div, 1111.0),
        ),
    ] {
        let actual = parse_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

#[test]
fn precedence() {
    use ast::{self, BinaryOp::*};

    for (name, input, expected) in [
        (
            "+ then +",
            "1 + 2 + 3",
            mk_op(mk_op(1.0, Add, 2.0), Add, 3.0),
        ),
        (
            "+ then -",
            "3 + 5 - 4",
            mk_op(mk_op(3.0, Add, 5.0), Sub, 4.0),
        ),
        (
            "+ then *",
            "1 + 2 * 3",
            mk_op(1.0, Add, mk_op(2.0, Mul, 3.0)),
        ),
        (
            "+ then /",
            "2 + 2 / 2",
            mk_op(2.0, Add, mk_op(2.0, Div, 2.0)),
        ),
        (
            "- then +",
            "6 - 2 + 1",
            mk_op(mk_op(6.0, Sub, 2.0), Add, 1.0),
        ),
        (
            "- then -",
            "9 - 6 - 8",
            mk_op(mk_op(9.0, Sub, 6.0), Sub, 8.0),
        ),
        (
            "- then *",
            "3 - 2 * 1",
            mk_op(3.0, Sub, mk_op(2.0, Mul, 1.0)),
        ),
        (
            "- then /",
            "2 - 2 / 2",
            mk_op(2.0, Sub, mk_op(2.0, Div, 2.0)),
        ),
        (
            "* then +",
            "1 * 2 + 3",
            mk_op(mk_op(1.0, Mul, 2.0), Add, 3.0),
        ),
        (
            "* then -",
            "3 * 5 - 4",
            mk_op(mk_op(3.0, Mul, 5.0), Sub, 4.0),
        ),
        (
            "* then *",
            "1 * 2 * 3",
            mk_op(mk_op(1.0, Mul, 2.0), Mul, 3.0),
        ),
        (
            "* then /",
            "2 * 2 / 2",
            mk_op(mk_op(2.0, Mul, 2.0), Div, 2.0),
        ),
        (
            "/ then +",
            "6 / 2 + 1",
            mk_op(mk_op(6.0, Div, 2.0), Add, 1.0),
        ),
        (
            "/ then -",
            "9 / 6 - 8",
            mk_op(mk_op(9.0, Div, 6.0), Sub, 8.0),
        ),
        (
            "/ then *",
            "3 / 2 * 1",
            mk_op(mk_op(3.0, Div, 2.0), Mul, 1.0),
        ),
        (
            "/ then /",
            "2 / 2 / 2",
            mk_op(mk_op(2.0, Div, 2.0), Div, 2.0),
        ),
        (
            "* then + then *",
            "2 * 2 + 3 * 3",
            mk_op(mk_op(2.0, Mul, 2.0), Add, mk_op(3.0, Mul, 3.0)),
        ),
        (
            "+ then * then +",
            "2 + 2 * 3 + 3",
            mk_op(mk_op(2.0, Add, mk_op(2.0, Mul, 3.0)), Add, 3.0),
        ),
        (
            "/ then * then /",
            "2 / 4 * 1 / 3",
            mk_op(mk_op(mk_op(2.0, Div, 4.0), Mul, 1.0), Div, 3.0),
        ),
        (
            "parens can force precedence",
            "2 / 4 * (1 / 3)",
            mk_op(mk_op(2.0, Div, 4.0), Mul, mk_op(1.0, Div, 3.0)),
        ),
    ] {
        let expr = parse_successfully(input);
        assert_eq!(expected, expr, "{}", name)
    }
}

#[test]
fn typechecking() {
    for (name, input, expected) in [
        ("addition", "1 + 1", Type::Num),
        ("subtraction", "99 - 4", Type::Num),
        ("multiplication", "-3 * -914", Type::Num),
        ("division", "4444 / 1111", Type::Num),
    ] {
        let actual = typecheck_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

#[test]
fn typechecking_fail() {
    for (name, input, expected) in [
        ("non-Num rhs", "1 + ()", TypeError::Mismatch),
        ("non-Num lhs", "() / 9", TypeError::Mismatch),
    ] {
        let actual = check_types(input);
        assert_eq!(Err(Error::TypeError(expected)), actual, "{}", name)
    }
}

#[test]
fn evaluation() {
    for (name, input, expected) in [
        ("simple addition", "1 + 1", Ok(Val::Num(2.0))),
        ("neg addition", "-9 + -44", Ok(Val::Num(-53.0))),
        ("simple subtraction", "5 - 3", Ok(Val::Num(2.0))),
        ("neg subtraction", "-24 - -4", Ok(Val::Num(-20.0))),
        ("simple multiplication", "33 * 3", Ok(Val::Num(99.0))),
        ("neg multiplication", "-44 * -4", Ok(Val::Num(176.0))),
        ("simple divion", "4 / 2", Ok(Val::Num(2.0))),
        ("negative divion", "-99 / -3", Ok(Val::Num(33.0))),
        (
            "division by zero",
            "0 / 0",
            Err(Error::EvaluationError(EvaluationError::DivisionByZero)),
        ),
    ] {
        let actual = evaluate(input);
        assert_eq!(expected, actual, "{}", name)
    }
}
