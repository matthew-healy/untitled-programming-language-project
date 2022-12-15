use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};
use test_generator::test_resources;
use untitled_programming_language_project::{evaluate, values::Val};

#[test_resources("./examples/pass/**/*.uplp")]
pub fn test_example_file(p: &str) {
    let path = {
        let proj_root = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(proj_root).join(p)
    };

    let file_contents = {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    let (annotation, program) = file_contents
        .split_once('\n')
        .expect("Could not extract expectation from test file.");

    let a = annotation
        .strip_prefix("-- test: ")
        .expect("Malformed test annotation");

    match a {
        "skip" => (),
        s if s.starts_with("check ") => {
            let json = s.strip_prefix("check ").unwrap();
            let e: TestExpectation =
                serde_json::from_str(json).expect("Malformed test annotation json");

            let result = evaluate(program).expect("Program evaluation failed");

            assert_eq!(Val::from(e), result);
        }
        _ => panic!("Unsupported test annotation."),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum TestExpectation {
    Num(f64),
    Unit {},
}

impl From<TestExpectation> for Val {
    fn from(e: TestExpectation) -> Self {
        match e {
            TestExpectation::Num(n) => Val::Num(n),
            TestExpectation::Unit {} => Val::Unit,
        }
    }
}
