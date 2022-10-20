use rand::Rng;
use untitled_programming_language_project::{ast::Expr, types::Type, values::Val};

pub mod common;
use common::{evaluate_successfully, parse_successfully, typecheck_successfully};

#[test]
fn parsing() {
    with_random_ints(|n| {
        let input = format!("{}", n);
        let expr = parse_successfully(input.as_str());
        assert_eq!(Expr::Number(n), expr)
    })
}

#[test]
fn typechecking() {
    with_random_ints(|n| {
        let input = format!("{}", n);
        let ty = typecheck_successfully(input.as_str());
        assert_eq!(Type::Num, ty)
    });
}

#[test]
fn evaluation() {
    with_random_ints(|n| {
        let input = format!("{}", n);
        let val = evaluate_successfully(input.as_str());
        assert_eq!(Val::Num(n), val)
    })
}

fn with_random_ints(f: fn(i32) -> ()) {
    let mut rng = rand::thread_rng();

    let mut ns = [0; 128];
    rng.fill(&mut ns);

    for n in ns {
        f(n)
    }
}
