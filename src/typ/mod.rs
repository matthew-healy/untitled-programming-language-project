use std::fmt;

use crate::{ast::Expr, error, interner};

use self::ctx::Ctx;

mod checker;
mod ctx;

/// Infer the `Type` of `e`, or return an appropriate `Error`.
pub fn infer(e: &Expr) -> Result<Type, Error> {
    let (t, ctx) = checker::synthesize_type(&mut checker::State::new(), Ctx::new(), e)?;
    let t = t.apply(&ctx);
    Ok(t)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IllFormedType(Type),
    Mismatch { got: Type, expected: Type },
    InvalidApplication(Type),
    UnboundVariable(interner::Id),
    Internal(String),
}

impl From<Error> for error::Error {
    fn from(e: Error) -> Self {
        error::Error::TypeError(e)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    Existential(Existential),
    Primitive(Primitive),
}

impl Type {
    pub fn bool() -> Self {
        Self::Primitive(Primitive::Bool)
    }

    pub fn num() -> Self {
        Self::Primitive(Primitive::Num)
    }

    pub fn unit() -> Self {
        Self::Primitive(Primitive::Unit)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Arrow(t1, t2) => write!(f, "{t1} -> {t2}"),
            Type::Existential(n) => write!(f, "{n}"),
            Type::Primitive(p) => write!(f, "{p}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Existential(usize);

impl fmt::Display for Existential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primitive {
    Bool,
    Num,
    Unit,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Bool => write!(f, "Bool"),
            Primitive::Num => write!(f, "Num"),
            Primitive::Unit => write!(f, "Unit"),
        }
    }
}
