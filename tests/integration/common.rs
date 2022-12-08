use std::{path::{Path, PathBuf}, env, fs::File, io::Read, str::FromStr};

use untitled_programming_language_project::{
    ast::{BinaryOp, RawExpr},
    check_types, evaluate, parse,
    types::Type,
    values::Val,
};

pub fn parse_successfully<'input>(input: &'input str) -> RawExpr {
    *parse(input).unwrap_or_else(|e| {
        panic!(
            "unexpected parse failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

pub fn typecheck_successfully<'input>(input: &'input str) -> Type {
    check_types(input).unwrap_or_else(|e| {
        panic!(
            "unexpected typecheck failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

pub fn evaluate_successfully<'input>(input: &'input str) -> Val {
    evaluate(input).unwrap_or_else(|e| {
        panic!(
            "unexpected evaluation failure.\ninput: {}\nerror: {:?}",
            input, e
        )
    })
}

/// make a binary op from two expressions.
pub fn mk_op<L, R>(l: L, op: BinaryOp, r: R) -> RawExpr
where
    L: Into<RawExpr>,
    R: Into<RawExpr>,
{
    RawExpr::Op(Box::new(l.into()), op, Box::new(r.into()))
}

pub fn test_example_file(p: &Path) {
    let path = {
        let proj_root = env::var("CARGO_MANIFEST_DIR")
            .expect("Could not get CARGO_MANIFEST_DIR");
        PathBuf::from(proj_root).join(p)
    };

    let file_contents = {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    let (expectation, program) = file_contents.split_once('\n')
        .expect("Could not extract expectation from test file.");

    let expected = expectation.strip_prefix("-- EXPECTED: ")
        .and_then(|e| f64::from_str(e).ok())
        .expect("Expectation was not of the correct format.");

    let result = evaluate(program)
        .expect("Program evaluation failed");

    assert_eq!(Val::Num(expected), result);
}
