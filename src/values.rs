use std::{fmt::Display, cell::RefCell};

use crate::{env::Env, error::EvaluationError, vm::Op};

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Bool(bool),
    Closure { body: Vec<Op>, env: Env<RefCell<Val>> },
    Dummy,
    Num(f64),
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Bool(b) => write!(f, "{}", b),
            Val::Closure { .. } => write!(f, "<function>"),
            Val::Dummy => write!(f, "<dummy>"),
            Val::Num(n) => write!(f, "{}", n),
            Val::Unit => write!(f, "()"),
        }
    }
}

impl Val {
    pub fn as_num(self) -> Result<f64, EvaluationError> {
        match self {
            Val::Num(n) => Ok(n),
            v => Err(EvaluationError::Internal(format!(
                "expected Num, got {:?}",
                v
            ))),
        }
    }

    pub fn as_bool(self) -> Result<bool, EvaluationError> {
        match self {
            Val::Bool(b) => Ok(b),
            v => Err(EvaluationError::Internal(format!(
                "expected Bool, got {:?}",
                v
            ))),
        }
    }

    pub fn try_eq(&self, other: &Self) -> Result<bool, EvaluationError> {
        match (self, other) {
            (Val::Bool(l), Val::Bool(r)) => Ok(l == r),
            (Val::Num(l), Val::Num(r)) => Ok(l == r),
            (Val::Unit, Val::Unit) => Ok(true),
            (Val::Closure { .. }, Val::Closure { .. }) => Err(EvaluationError::IllegalEquality),
            (_, _) => Ok(false),
        }
    }
}
