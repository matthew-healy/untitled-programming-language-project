use std::fmt::Display;

use crate::error::EvaluationError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Val {
    Num(f64),
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
