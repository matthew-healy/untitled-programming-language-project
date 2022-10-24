use crate::{
    ast::{Expr, Opcode},
    error::EvaluationError,
    values::Val,
};

pub(crate) struct VirtualMachine;

impl VirtualMachine {
    pub(crate) fn new() -> Self {
        VirtualMachine
    }

    pub(crate) fn evaluate(&self, expr: Expr) -> Result<Val, EvaluationError> {
        match expr {
            Expr::Number(n) => Ok(Val::Num(n)),
            Expr::Op(l, op, r) => {
                let l = self.evaluate_num(*l)?;
                let r = self.evaluate_num(*r)?;
                match op {
                    Opcode::Add => Ok(Val::Num(l + r)),
                    Opcode::Sub => Ok(Val::Num(l - r)),
                    Opcode::Mul => Ok(Val::Num(l * r)),
                    Opcode::Div if r == 0 => Err(EvaluationError::DivisionByZero),
                    Opcode::Div => Ok(Val::Num(l / r)),
                }
            }
            Expr::Unit => Ok(Val::Unit),
        }
    }

    fn evaluate_num(&self, e: Expr) -> Result<i32, EvaluationError> {
        self.evaluate(e).and_then(|e| match e {
            Val::Num(n) => Ok(n),
            v => Err(EvaluationError::Internal(format!(
                "Expected Num but found {:?}",
                v
            ))),
        })
    }
}
