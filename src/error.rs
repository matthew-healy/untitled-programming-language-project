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

impl<'src> From<LalrpopError<'src>> for Error {
    fn from(e: LalrpopError<'src>) -> Self {
        Error::ParseError(e.into())
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl<'src> From<LalrpopError<'src>> for ParseError {
    fn from(e: LalrpopError<'src>) -> Self {
        use lalrpop_util::ParseError::*;

        match e {
            InvalidToken { location } => ParseError::InvalidToken { location },
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
            User { .. } => unreachable!("We don't currently use lalrpop's user error feature."),
        }
    }
}

// Type Errors

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    Mismatch { t1: Type, t2: Type },
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
