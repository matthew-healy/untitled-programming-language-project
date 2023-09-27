use ast::{Expr, RawExpr};
use error::Error;
use lalrpop_util::{self, lalrpop_mod};
use parser::UplpParser;
use scopes::ScopeChecker;
use typ::Type;

pub mod ast;
mod env;
pub mod error;
mod interner;
mod scopes;
pub mod typ;
pub mod values;
mod vm;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub parser
);

pub fn parse(input: &str) -> Result<Box<RawExpr>, Error> {
    let parser = UplpParser::new();
    let expr = parser
        .parse(input)
        .map_err(|e| Error::from_lalrpop(e, input))?;
    Ok(expr)
}

pub fn check_types(input: &str) -> Result<Type, Error> {
    let mut type_checker = typ::check::er();

    let expr = parse_and_scope_check(input)?;
    let typ = type_checker.infer(&expr)?;
    Ok(typ)
}

pub fn evaluate(input: &str) -> Result<values::Val, error::Error> {
    let expr = parse_and_scope_check(input)?;

    let mut type_checker = typ::check::er();
    let _ = type_checker.infer(&expr)?;

    let compiler = vm::Compiler::new();
    let code = compiler.compile(&expr);

    let mut vm = vm::VirtualMachine::new(code);

    let val = vm.evaluate()?;
    Ok(val)
}

fn parse_and_scope_check(input: &str) -> Result<Box<Expr>, Error> {
    let expr = parse(input)?;

    let mut scope_checker = ScopeChecker::new();
    let expr = scope_checker.check(*expr)?;

    Ok(Box::new(expr))
}
