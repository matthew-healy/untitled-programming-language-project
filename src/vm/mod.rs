use crate::{ast::Opcode, error::EvaluationError, values::Val};

mod compiler;
mod stack;

use self::stack::Stack;
pub use compiler::Compiler;
use compiler::Op;

pub struct VirtualMachine {
    code: Vec<Op>,
    stack: Stack<Val>,
}

impl VirtualMachine {
    pub fn new(code: Vec<Op>) -> Self {
        let stack = Stack::new();
        VirtualMachine { code, stack }
    }

    pub fn evaluate(&mut self) -> Result<Val, EvaluationError> {
        while let Some(o) = self.code.pop() {
            match o {
                Op::Const(v) => self.stack.push(v),
                Op::Binary(op) => {
                    let r = self.stack.force_pop_num()?;
                    let l = self.stack.force_pop_num()?;

                    let res = match op {
                        Opcode::Add => l + r,
                        Opcode::Mul => l * r,
                        Opcode::Sub => l - r,
                        Opcode::Div => {
                            if r == 0 {
                                Err(EvaluationError::DivisionByZero)
                            } else {
                                Ok(l / r)
                            }
                        }?,
                    };

                    self.stack.push(Val::Num(res));
                }
            }
        }

        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            Err(EvaluationError::Internal(
                "unexpected end of program".into(),
            ))
        }
    }
}

impl Stack<Val> {
    fn force_pop_num(&mut self) -> Result<i32, EvaluationError> {
        match self.pop() {
            Some(Val::Num(n)) => Ok(n),
            v => Err(EvaluationError::Internal(format!(
                "Expected Num but got {:?}",
                v
            ))),
        }
    }
}
