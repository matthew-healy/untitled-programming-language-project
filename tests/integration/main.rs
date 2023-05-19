use serde::Deserialize;
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
pub fn test(p: &str) {
    let test = parse_annotated_test(p);

    match test.expectation {
        Expectation::Skip => (),
        Expectation::Value(v) => {
            let result = evaluate(test.program.as_str()).expect("Program evaluation failed");
            assert_eq!(v, result)
        }
        Expectation::Error(e) => {
            let result = evaluate(test.program.as_str()).expect_err("Nothing went wrong");
            assert_eq!(e, result)
        }
    }
}

struct AnnotatedTest {
    expectation: Expectation,
    program: String,
}

fn parse_annotated_test(p: &str) -> AnnotatedTest {
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

    AnnotatedTest {
        program,
        expectation,
    }
}

#[derive(Deserialize)]
#[serde(tag = "category", content = "metadata")]
enum Expectation {
    #[serde(rename = "value")]
    Value(ValueExpectation),
    #[serde(rename = "error")]
    Error(ErrorExpectation),
    #[serde(rename = "skip")]
    Skip,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
enum ValueExpectation {
    Bool(bool),
    Num(f64),
    Unit,
    Closure,
}

impl PartialEq<Val> for ValueExpectation {
    fn eq(&self, other: &Val) -> bool {
        match (self, other) {
            (ValueExpectation::Bool(b1), Val::Bool(b2)) => b1 == b2,
            (ValueExpectation::Num(n1), Val::Num(n2)) => n1 == n2,
            (ValueExpectation::Unit, Val::Unit) => true,
            (ValueExpectation::Closure, Val::Closure { .. }) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize)]
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

impl PartialEq<Error> for ErrorExpectation {
    fn eq(&self, other: &Error) -> bool {
        use ErrorExpectation::*;

        match (self, other) {
            (
                UnboundVar { ident: ident1 },
                Error::ParseError(ParseError::UnboundIdentifier { ident: ident2 }),
            ) => ident1.as_str() == ident2.as_str(),
            (
                UnexpectedToken { tok: tok1 },
                Error::ParseError(ParseError::UnexpectedToken {
                    token: Tok::Raw(tok2),
                    ..
                }),
            ) => tok1.as_str() == tok2,
            (
                UnexpectedToken { tok },
                Error::ParseError(ParseError::UnexpectedToken {
                    token: Tok::EndOfFile,
                    ..
                }),
            ) => tok == "eof",
            (
                InvalidToken { tok: tok1 },
                Error::ParseError(ParseError::InvalidToken { token: tok2, .. }),
            ) => tok1 == tok2,
            (
                TypeMismatch {
                    got: got1,
                    expected: expected1,
                },
                Error::TypeError(TypeError::Mismatch {
                    got: got2,
                    expected: expected2,
                }),
            ) => got1.as_str() == got2.to_string() && expected1.as_str() == expected2.to_string(),
            (DivisionByZero, Error::EvaluationError(EvaluationError::DivisionByZero)) => true,
            _ => false,
        }
    }
}
