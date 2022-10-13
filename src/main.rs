use lalrpop_util::lalrpop_mod;

mod ast;

lalrpop_mod!(parser);

fn main() {
    println!("solo version 0.1.0");

    println!(
        "{:?}",
        parser::ExprParser::new()
            .parse("99 + 105 * 22 / 4")
            .unwrap()
    );
}
