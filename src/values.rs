use std::fmt::Display;

use crate::{env::Env, error::EvaluationError, vm::Op};

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Bool(bool),
    Closure { body: Vec<Op>, env: Env<Val> },
    Num(f64),
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Bool(b) => write!(f, "{}", b),
            Val::Closure { .. } => write!(f, "<function>"),
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
}
