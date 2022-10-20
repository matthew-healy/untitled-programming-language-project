use rand::Rng;
use untitled_programming_language_project::{ast, types::Type};

pub mod common;
use common::{parse_successfully, typecheck_successfully};

#[test]
fn parsing() {
    let mut rng = rand::thread_rng();

    let mut ns = [0; 128];
    rng.fill(&mut ns);

    for n in ns {
        let input = format!("{}", n);
        let expr = parse_successfully(input.as_str());
        assert_eq!(ast::Expr::Number(n), expr)
    }
}

#[test]
fn typechecking() {
    let mut rng = rand::thread_rng();

    let mut ns = [0; 128];
    rng.fill(&mut ns);

    for n in ns {
        let input = format!("{}", n);
        let ty = typecheck_successfully(input.as_str());
        assert_eq!(Type::Num, ty)
    }
}