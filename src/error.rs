use crate::parser::Token;
use lalrpop_util;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    TypeError(TypeError),
}

// Parse errors

#[derive(Debug)]
pub enum ParseError {
    InvalidToken {
        location: usize,
    },
    UnexpectedEndOfFile {
        location: usize,
        expected: Vec<String>,
    },
    UnexpectedToken {
        location: usize,
        token: Tok,
        expected: Vec<String>,
    },
}

#[derive(Debug)]
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
            User { .. } => unreachable!(),
        }
    }
}

// Type Errors

#[derive(Debug)]
pub enum TypeError {
    Mismatch,
}

impl From<TypeError> for Error {
    fn from(e: TypeError) -> Self {
        Error::TypeError(e)
    }
}