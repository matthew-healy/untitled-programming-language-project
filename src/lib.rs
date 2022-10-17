use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod types;
lalrpop_mod!(pub parser);
