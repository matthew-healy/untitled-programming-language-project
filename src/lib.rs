use lalrpop_util::{self, lalrpop_mod};

pub mod ast;
pub mod error;
pub mod types;
lalrpop_mod!(pub parser);

pub fn check_types<'input>(input: &'input str) -> Result<types::Type, error::Error> {
    let parser = parser::ExprParser::new();
    let typechecker = types::TypeChecker::new();

    let expr = parser.parse(input)?;
    let typ = typechecker.check(*expr)?;
    Ok(typ)
}
