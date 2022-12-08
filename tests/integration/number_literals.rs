use std::fmt::Display;

use quickcheck::{quickcheck, Arbitrary};
use untitled_programming_language_project::{ast::RawExpr, types::Type, values::Val};

use crate::common::{evaluate_successfully, parse_successfully, typecheck_successfully};

quickcheck! {
    fn parsing(n: ValidNum) -> bool {
        let input = format!("{}", n);
        let expr = parse_successfully(input.as_str());
        RawExpr::Literal(n.into()) == expr
    }
}

quickcheck! {
    fn typechecking(n: ValidNum) -> bool {
        let input = format!("{}", n);
        let ty = typecheck_successfully(input.as_str());
        Type::Num == ty
    }
}

quickcheck! {
    fn evaluation(n: ValidNum) -> bool {
        let input = format!("{}", n);
        let val = evaluate_successfully(input.as_str());
        val == n.into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ValidNum(f64);

impl From<ValidNum> for Val {
    fn from(n: ValidNum) -> Self {
        Val::Num(n.0)
    }
}

impl Display for ValidNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Arbitrary for ValidNum {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Generate arbitrary f64s until we one that
        // is neither infinite nor NaN.
        ValidNum(loop {
            let a = f64::arbitrary(g);
            if a.is_finite() {
                break a;
            }
        })
    }
}
