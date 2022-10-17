use untitled_programming_language_project::parser;
use untitled_programming_language_project::types;

fn main() {
    println!("untitled programming language project, version 0.1.0");

    let input = "99 + 105 * 22 / 4";

    let parser = parser::ExprParser::new();
    let typechecker = types::TypeChecker::new();

    match parser.parse(input) {
        Err(err) => println!("Parsing error: {}", err),
        Ok(expr) => {
            println!("Parsed: {:?}", expr);
            match typechecker.check(*expr) {
                Err(err) => println!("Type error: {:?}", err),
                Ok(ty) => println!("Typechecked: {:?}", ty),
            }
        }
    };
}
