use untitled_programming_language_project::{ast::Expr, types::Type, values::Val};

pub mod common;
use common::{evaluate_successfully, parse_successfully, typecheck_successfully};

#[test]
fn parsing() {
    assert_eq!(Expr::Unit, parse_successfully("()"))
}

#[test]
fn typechecking() {
    assert_eq!(Type::Unit, typecheck_successfully("()"))
}

#[test]
fn evalutation() {
    assert_eq!(Val::Unit, evaluate_successfully("()"))
}
