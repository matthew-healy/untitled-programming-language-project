use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};
use test_generator::test_resources;
use untitled_programming_language_project::{
    error::{Error, EvaluationError, ParseError, Tok, TypeError},
    evaluate,
    values::Val,
};

#[test_resources("./examples/*/*.uplp")]
pub fn test_error_file(p: &str) {
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
        s if s.starts_with("value ") => {
            let json = s.strip_prefix("value ").unwrap();
            let e: ValueExpectation =
                serde_json::from_str(json).expect("Malformed test annotation json");

            let result = evaluate(program).expect("Program evaluation failed");

            assert_eq!(Val::from(e), result);
        }
        s if s.starts_with("error ") => {
            use ErrorExpectation::*;

            let json = s.strip_prefix("error ").unwrap();
            let e: ErrorExpectation =
                serde_json::from_str(json).expect("Malformed test annotation json");

            let result = evaluate(program).expect_err("Nothing went wrong");

            match (e, result) {
                (
                    UnboundVar { ident: expected },
                    Error::ParseError(ParseError::UnboundIdentifier { ident: actual }),
                ) => assert_eq!(expected, *actual),
                (
                    UnexpectedToken {
                        loc: loc1,
                        tok: tok1,
                    },
                    Error::ParseError(ParseError::UnexpectedToken {
                        location: loc2,
                        token: tok2,
                        ..
                    }),
                ) => {
                    assert_eq!(loc1, loc2);
                    match tok2 {
                        Tok::EndOfFile => assert_eq!(tok1, "eof"),
                        Tok::Raw(tok2) => assert_eq!(tok1, tok2),
                    };
                }
                (
                    InvalidToken { loc: loc1 },
                    Error::ParseError(ParseError::InvalidToken { location: loc2 }),
                ) => assert_eq!(loc1, loc2),
                (DivisionByZero, Error::EvaluationError(EvaluationError::DivisionByZero)) => (),
                (ErrorExpectation::TypeMismatch, Error::TypeError(TypeError::Mismatch)) => (),
                (e, err) => panic!("Unrecognised test expectation {e:?}. Got {err:?}"),
            }
        }
        _ => panic!("Unsupported test annotation."),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum ValueExpectation {
    Bool(bool),
    Num(f64),
    Unit {},
}

impl From<ValueExpectation> for Val {
    fn from(e: ValueExpectation) -> Self {
        match e {
            ValueExpectation::Bool(b) => Val::Bool(b),
            ValueExpectation::Num(n) => Val::Num(n),
            ValueExpectation::Unit {} => Val::Unit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "category", content = "metadata")]
enum ErrorExpectation {
    #[serde(rename = "Parse.unbound_var")]
    UnboundVar { ident: String },
    #[serde(rename = "Parse.unexpected_token")]
    UnexpectedToken { loc: usize, tok: String },
    #[serde(rename = "Parse.invalid_token")]
    InvalidToken { loc: usize },
    #[serde(rename = "Type.mismatch")]
    TypeMismatch,
    #[serde(rename = "Evaluation.division_by_zero")]
    DivisionByZero,
}
