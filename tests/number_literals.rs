use quickcheck::quickcheck;
use untitled_programming_language_project::{ast::RawExpr, types::Type, values::Val};

pub mod common;
use common::{evaluate_successfully, parse_successfully, typecheck_successfully};

quickcheck! {
    fn parsing(n: i32) -> bool {
        let input = format!("{}", n);
        let expr = parse_successfully(input.as_str());
        RawExpr::Literal(Val::Num(n)) == expr
    }
}

quickcheck! {
    fn typechecking(n: i32) -> bool {
        let input = format!("{}", n);
        let ty = typecheck_successfully(input.as_str());
        Type::Num == ty
    }
}

quickcheck! {
    fn evaluation(n: i32) -> bool {
        let input = format!("{}", n);
        let val = evaluate_successfully(input.as_str());
        Val::Num(n) == val
    }
}
