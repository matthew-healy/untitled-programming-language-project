use ast::{RawExpr, Expr};
use error::Error;
use lalrpop_util::{self, lalrpop_mod};
use parser::ExprParser;
use scopes::ScopeChecker;
use types::{TypeChecker, Type};

pub mod ast;
pub mod error;
pub mod types;
pub mod values;
mod env;
mod scopes;
mod vm;

lalrpop_mod!(
    #[allow(clippy::all)]
    parser
);

pub fn parse(input: &str) -> Result<Box<RawExpr>, Error> {
    let parser = ExprParser::new();
    let expr = parser.parse(input)?;
    Ok(expr)
}

pub fn check_types(input: &str) -> Result<Type, Error> {
    let mut type_checker = types::TypeChecker::new();

    let expr = parse_and_scope_check(input)?;
    let typ = type_checker.check(&expr)?;
    Ok(typ)
}

pub fn evaluate(input: &str) -> Result<values::Val, error::Error> {
    let expr = parse_and_scope_check(input)?;

    let mut type_checker = TypeChecker::new();
    let _ = type_checker.check(&expr)?;

    let compiler = vm::Compiler::new();
    let stack = compiler.compile(&expr);

    let mut vm = vm::VirtualMachine::new(stack);

    let val = vm.evaluate()?;
    Ok(val)
}

fn parse_and_scope_check(input: &str) -> Result<Box<Expr>, Error> {
    let expr = parse(input)?;

    let mut scope_checker = ScopeChecker::new();
    let expr = scope_checker.check(*expr)?;

    Ok(Box::new(expr))
}