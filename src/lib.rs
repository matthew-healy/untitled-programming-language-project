use lalrpop_util::{self, lalrpop_mod};

pub mod ast;
pub mod error;
pub mod types;
pub mod values;
mod vm;

lalrpop_mod!(
    #[allow(clippy::all)]
    parser
);

pub fn parse(input: &str) -> Result<Box<ast::Expr>, error::Error> {
    let parser = parser::ExprParser::new();
    let expr = parser.parse(input)?;
    Ok(expr)
}

pub fn check_types(input: &str) -> Result<types::Type, error::Error> {
    let typechecker = types::TypeChecker::new();

    let expr = parse(input)?;
    let typ = typechecker.check(&expr)?;
    Ok(typ)
}

pub fn evaluate(input: &str) -> Result<values::Val, error::Error> {
    let typechecker = types::TypeChecker::new();

    let expr = parse(input)?;
    let _ = typechecker.check(&expr)?;

    let compiler = vm::Compiler::new();
    let stack = compiler.compile(&expr);

    let mut vm = vm::VirtualMachine::new(stack);

    let val = vm.evaluate()?;
    Ok(val)
}
