use crate::{ast::RawIdent, parser::Token, types::Type};
use lalrpop_util;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseError(ParseError),
    TypeError(TypeError),
    EvaluationError(EvaluationError),
}

// Parse errors

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidToken {
        token: String,
        location: usize,
    },
    UnexpectedToken {
        location: usize,
        token: Tok,
        expected: Vec<String>,
    },
    UnboundIdentifier {
        ident: RawIdent,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tok {
    EndOfFile,
    Raw(String),
}

type LalrpopError<'src> = lalrpop_util::ParseError<usize, Token<'src>, &'static str>;

impl Error {
    pub fn from_lalrpop<'src>(e: LalrpopError<'src>, src: &'src str) -> Self {
        use lalrpop_util::ParseError::*;

        let parse_error = match e {
            InvalidToken { location } => ParseError::InvalidToken {
                token: src[location..location + 1].to_owned(),
                location,
            },
            UnrecognizedEOF { location, expected } => ParseError::UnexpectedToken {
                location,
                token: Tok::EndOfFile,
                expected,
            },
            UnrecognizedToken {
                token: (location, tok, _),
                expected,
            } => ParseError::UnexpectedToken {
                location,
                token: Tok::Raw(tok.to_string()),
                expected,
            },
            ExtraToken {
                token: (location, tok, _),
            } => ParseError::UnexpectedToken {
                location,
                token: Tok::Raw(tok.to_string()),
                expected: vec![],
            },
            User { .. } => unreachable!("We don't currently use lalrpop's user error feature"),
        };

        Self::ParseError(parse_error)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

// Type Errors

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    Mismatch { got: Type, expected: Type },
    BadApplication,
}

impl From<TypeError> for Error {
    fn from(e: TypeError) -> Self {
        Error::TypeError(e)
    }
}

// Evaluation Errors

#[derive(Debug, PartialEq, Eq)]
pub enum EvaluationError {
    DivisionByZero,
    IllegalEquality,
    Internal(String),
}

impl From<EvaluationError> for Error {
    fn from(e: EvaluationError) -> Self {
        Error::EvaluationError(e)
    }
}
