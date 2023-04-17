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
    let mut preface = Vec::new();
    let mut program = Vec::new();

    let mut split = false;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line == "---" {
            split = true;
            continue;
        }

        if split {
            program.push(line);
        } else {
            preface.push(line);
        }
    }
    if !split {
        std::mem::swap(&mut program, &mut preface);
    }

    let expectation = preface.join("\n");
    let expectation: Expectation = toml::from_str(expectation.as_str()).unwrap();

    let program = program.join("\n");

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
                (
                    ErrorExpectation::TypeMismatch { t1: t11, t2: t21 },
                    Error::TypeError(TypeError::Mismatch { t1: t12, t2: t22 }),
                ) => {
                    assert_eq!(t11, format!("{t12}"));
                    assert_eq!(t21, format!("{t22}"));
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
    UnexpectedToken { loc: usize, tok: String },
    #[serde(rename = "Parse.invalid_token")]
    InvalidToken { loc: usize },
    #[serde(rename = "Type.mismatch")]
    TypeMismatch { t1: String, t2: String },
    #[serde(rename = "Evaluation.division_by_zero")]
    DivisionByZero,
}
