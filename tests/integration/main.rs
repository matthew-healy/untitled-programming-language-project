use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};
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

    let file = File::open(path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut preface = String::new();
    let mut program = String::new();

    let mut lines = reader.lines();

    let mut in_preface = true;

    while let Some(Ok(ln)) = lines.next() {
        program.push_str(ln.as_str());
        program.push('\n');

        if in_preface {
            if ln.starts_with("--") {
                let uncommented = if ln.len() > 2 { &ln[3..] } else { "" };
                preface.push_str(uncommented);
                preface.push('\n');
            } else {
                in_preface = false;
            }
        }
    }

    if preface.is_empty() {
        std::mem::swap(&mut program, &mut preface);
    }

    let expectation: Expectation =
        toml::from_str(preface.as_str()).expect("Failed to parse toml header");

    match expectation {
        Expectation::Skip => (),
        Expectation::Value(v) => {
            let result = evaluate(program.as_str()).expect("Program evaluation failed");
            assert_eq!(Val::from(v), result)
        }
        Expectation::Error(e) => {
            use ErrorExpectation::*;

            let result = evaluate(program.as_str()).expect_err("Nothing went wrong");

            match (e, result) {
                (
                    UnboundVar { ident: expected },
                    Error::ParseError(ParseError::UnboundIdentifier { ident: actual }),
                ) => assert_eq!(expected, *actual),
                (
                    UnexpectedToken { tok: tok1 },
                    Error::ParseError(ParseError::UnexpectedToken { token: tok2, .. }),
                ) => {
                    match tok2 {
                        Tok::EndOfFile => assert_eq!(tok1, "eof"),
                        Tok::Raw(tok2) => assert_eq!(tok1, tok2),
                    };
                }
                (
                    InvalidToken { tok: tok1 },
                    Error::ParseError(ParseError::InvalidToken { token: tok2, .. }),
                ) => assert_eq!(tok1, tok2),
                (DivisionByZero, Error::EvaluationError(EvaluationError::DivisionByZero)) => (),
                (
                    ErrorExpectation::TypeMismatch {
                        got: got1,
                        expected: expected1,
                    },
                    Error::TypeError(TypeError::Mismatch {
                        got: got2,
                        expected: expected2,
                    }),
                ) => {
                    assert_eq!(got1, format!("{got2}"));
                    assert_eq!(expected1, format!("{expected2}"));
                }
                (e, err) => panic!("Unrecognised test expectation {e:?}. Got {err:?}"),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "category", content = "metadata")]
enum Expectation {
    #[serde(rename = "value")]
    Value(ValueExpectation),
    #[serde(rename = "error")]
    Error(ErrorExpectation),
    #[serde(rename = "skip")]
    Skip,
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
#[serde(tag = "error", content = "expectation")]
enum ErrorExpectation {
    #[serde(rename = "Parse.unbound_var")]
    UnboundVar { ident: String },
    #[serde(rename = "Parse.unexpected_token")]
    UnexpectedToken { tok: String },
    #[serde(rename = "Parse.invalid_token")]
    InvalidToken { tok: String },
    #[serde(rename = "Type.mismatch")]
    TypeMismatch { got: String, expected: String },
    #[serde(rename = "Evaluation.division_by_zero")]
    DivisionByZero,
}
