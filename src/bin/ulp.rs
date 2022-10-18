use untitled_programming_language_project::check_types;

fn main() {
    println!("untitled programming language project, version 0.1.0");

    let input = "99 + 105 * 22 / 4";

    match check_types(input) {
        Err(err) => println!("{:?}", err),
        Ok(ty) => println!("Typechecked: {:?}", ty),
    }
}
