use lalrpop_util::{self, lalrpop_mod};

pub mod ast;
pub mod error;
pub mod types;
lalrpop_mod!(
    #[allow(clippy::all)]
    parser
);

pub fn parse(input: &str) -> Result<ast::Expr, error::Error> {
    let parser = parser::ExprParser::new();
    let expr = parser.parse(input)?;
    Ok(*expr)
}

pub fn check_types(input: &str) -> Result<types::Type, error::Error> {
    let typechecker = types::TypeChecker::new();

    let expr = parse(input)?;
    let typ = typechecker.check(expr)?;
    Ok(typ)
}
