use crate::error::EvaluationError;

#[derive(Debug, PartialEq)]
pub enum Val {
    Num(i32),
    Unit,
}

impl Val {
    pub fn as_num(self) -> Result<i32, EvaluationError> {
        match self {
            Val::Num(n) => Ok(n),
            v => Err(EvaluationError::Internal(format!(
                "expected Num, got {:?}",
                v
            ))),
        }
    }
}
