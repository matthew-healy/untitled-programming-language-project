use untitled_programming_language_project::{
    ast,
    error::{Error, EvaluationError},
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
        ("addition", "1 + 1", mk_op(1, Opcode::Add, 1)),
        ("subtraction", "99 - 4", mk_op(99, Opcode::Sub, 4)),
        ("multiplication", "-3 * -914", mk_op(-3, Opcode::Mul, -914)),
        ("division", "4444 / 1111", mk_op(4444, Opcode::Div, 1111)),
    ] {
        let actual = parse_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

#[test]
fn precedence() {
    use ast::{self, Opcode::*};

    for (name, input, expected) in [
        ("+ then +", "1 + 2 + 3", mk_op(mk_op(1, Add, 2), Add, 3)),
        ("+ then -", "3 + 5 - 4", mk_op(mk_op(3, Add, 5), Sub, 4)),
        ("+ then *", "1 + 2 * 3", mk_op(1, Add, mk_op(2, Mul, 3))),
        ("+ then /", "2 + 2 / 2", mk_op(2, Add, mk_op(2, Div, 2))),
        ("- then +", "6 - 2 + 1", mk_op(mk_op(6, Sub, 2), Add, 1)),
        ("- then -", "9 - 6 - 8", mk_op(mk_op(9, Sub, 6), Sub, 8)),
        ("- then *", "3 - 2 * 1", mk_op(3, Sub, mk_op(2, Mul, 1))),
        ("- then /", "2 - 2 / 2", mk_op(2, Sub, mk_op(2, Div, 2))),
        ("* then +", "1 * 2 + 3", mk_op(mk_op(1, Mul, 2), Add, 3)),
        ("* then -", "3 * 5 - 4", mk_op(mk_op(3, Mul, 5), Sub, 4)),
        ("* then *", "1 * 2 * 3", mk_op(mk_op(1, Mul, 2), Mul, 3)),
        ("* then /", "2 * 2 / 2", mk_op(mk_op(2, Mul, 2), Div, 2)),
        ("/ then +", "6 / 2 + 1", mk_op(mk_op(6, Div, 2), Add, 1)),
        ("/ then -", "9 / 6 - 8", mk_op(mk_op(9, Div, 6), Sub, 8)),
        ("/ then *", "3 / 2 * 1", mk_op(mk_op(3, Div, 2), Mul, 1)),
        ("/ then /", "2 / 2 / 2", mk_op(mk_op(2, Div, 2), Div, 2)),
        (
            "* then + then *",
            "2 * 2 + 3 * 3",
            mk_op(mk_op(2, Mul, 2), Add, mk_op(3, Mul, 3)),
        ),
        (
            "+ then * then +",
            "2 + 2 * 3 + 3",
            mk_op(mk_op(2, Add, mk_op(2, Mul, 3)), Add, 3),
        ),
        (
            "/ then * then /",
            "2 / 4 * 1 / 3",
            mk_op(mk_op(mk_op(2, Div, 4), Mul, 1), Div, 3),
        ),
        (
            "parens can force precedence",
            "2 / 4 * (1 / 3)",
            mk_op(mk_op(2, Div, 4), Mul, mk_op(1, Div, 3)),
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
fn evaluation() {
    for (name, input, expected) in [
        ("simple addition", "1 + 1", Ok(Val::Num(2))),
        ("neg addition", "-9 + -44", Ok(Val::Num(-53))),
        ("simple subtraction", "5 - 3", Ok(Val::Num(2))),
        ("neg subtraction", "-24 - -4", Ok(Val::Num(-20))),
        ("simple multiplication", "33 * 3", Ok(Val::Num(99))),
        ("neg multiplication", "-44 * -4", Ok(Val::Num(176))),
        ("simple divion", "4 / 2", Ok(Val::Num(2))),
        ("negative divion", "-99 / -3", Ok(Val::Num(33))),
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
