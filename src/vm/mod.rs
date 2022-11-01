// use std::collections::HashMap;

use crate::{ast::Opcode, env::Env, error::EvaluationError, values::Val};

mod compiler;
mod stack;

use self::stack::Stack;
pub use compiler::Compiler;

/// operations run by the vm.
pub enum Op {
    Access(usize),
    Binary(Opcode),
    Const(Val),
    Let(),
    EndLet(),
}

pub struct VirtualMachine {
    code: Vec<Op>,
    env: Env<Val>,
    stack: Stack<Val>,
}

impl VirtualMachine {
    pub fn new(code: Vec<Op>) -> Self {
        let env = Env::new();
        let stack = Stack::new();
        VirtualMachine { code, env, stack }
    }

    pub fn evaluate(&mut self) -> Result<Val, EvaluationError> {
        while let Some(o) = self.code.pop() {
            match o {
                Op::Const(v) => self.stack.push(v),
                // Strict binary operators
                Op::Binary(op) => {
                    let r = self.stack.force_pop_num()?;
                    let l = self.stack.force_pop_num()?;

                    let res = match op {
                        Opcode::Add => l + r,
                        Opcode::Mul => l * r,
                        Opcode::Sub => l - r,
                        Opcode::Div => {
                            if r == 0.0 {
                                Err(EvaluationError::DivisionByZero)
                            } else {
                                Ok(l / r)
                            }
                        }?,
                    };

                    self.stack.push(Val::Num(res));
                }
                Op::Access(i) => {
                    let v = self.env.lookup(i).ok_or(EvaluationError::Internal(format!(
                        // TODO: recover original variable name
                        "Attempt to access unbound variable {:?}",
                        i
                    )))?;
                    self.stack.push(*v);
                }
                Op::Let() => {
                    let v = self.stack.force_pop()?;
                    self.env.bind(v);
                }
                Op::EndLet() => {
                    self.env.unbind();
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
    fn force_pop(&mut self) -> Result<Val, EvaluationError> {
        match self.pop() {
            Some(v) => Ok(v),
            None => Err(EvaluationError::Internal(String::from(
                "Attempt to pop from empty stack",
            ))),
        }
    }

    fn force_pop_num(&mut self) -> Result<f64, EvaluationError> {
        match self.force_pop()? {
            Val::Num(n) => Ok(n),
            v => Err(EvaluationError::Internal(format!(
                "Expected Num but got {:?}",
                v
            ))),
        }
    }
}
