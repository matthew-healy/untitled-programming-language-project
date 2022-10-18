use rand::Rng;
use untitled_programming_language_project::{check_types, types::Type};

#[test]
fn number_literals() {
    let mut rng = rand::thread_rng();

    let mut ns = [0; 128];
    rng.fill(&mut ns);

    for n in ns {
        let input = format!("{}", n);
        let ty = typecheck_successfully(input.as_str());
        assert_eq!(Type::Num, ty)
    }
}

#[test]
fn binary_ops() {
    for (name, input, expected) in [
        ("addition", "1 + 1", Type::Num),
        ("subtraction", "99 - 4", Type::Num),
        ("multiplication", "-3 * -914", Type::Num),
        ("division", "4444 / 1111", Type::Num),
    ] {
        let actual = typecheck_successfully(input);
        assert_eq!(expected, actual, "{}", name)
    }
}

fn typecheck_successfully<'input>(input: &'input str) -> Type {
    check_types(input)
        .unwrap_or_else(|e| panic!("unexpected failure.\ninput: {}\nerror: {:?}", input, e))
}
