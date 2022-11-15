use std::{path::{PathBuf, Path}, fs::File, io::Read, env, str::FromStr};

use untitled_programming_language_project::{evaluate, values::Val};

#[test]
fn lambda() {
    test_example_file(Path::new("examples/pass/lambda.uplp"))
}

#[test]
fn lambda_arg_let() {
    test_example_file(Path::new("examples/pass/lambda_arg_let.uplp"))
}

#[test]
fn lambda_with_let() {
    test_example_file(Path::new("examples/pass/lambda_with_let.uplp"))
}

#[test]
fn multi_arg_application() {
    test_example_file(Path::new("examples/pass/multi_arg_application.uplp"))
}

fn test_example_file(p: &Path) {
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
