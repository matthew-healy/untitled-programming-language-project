use std::collections::HashMap;

use crate::{
    ast::{Ident, Opcode},
    error::EvaluationError,
    values::Val,
};

mod compiler;
mod stack;

use self::stack::Stack;
pub use compiler::Compiler;
use compiler::Op;

pub struct VirtualMachine {
    code: Vec<Op>,
    env: HashMap<Ident, Val>,
    stack: Stack<Val>,
}

impl VirtualMachine {
    pub fn new(code: Vec<Op>) -> Self {
        let env = HashMap::new();
        let stack = Stack::new();
        VirtualMachine { code, env, stack }
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
                Op::Access(i) => {
                    let v = self.env.get(&i).ok_or(EvaluationError::Internal(format!(
                        "Attempt to access unbound variable {:?}",
                        i
                    )))?;
                    self.stack.push(v.clone());
                }
                Op::Let(i) => {
                    let v = self.stack.force_pop()?;
                    self.env.insert(i, v);
                }
                Op::EndLet(i) => {
                    self.env.remove(&i);
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

    fn force_pop_num(&mut self) -> Result<i32, EvaluationError> {
        match self.force_pop()? {
            Val::Num(n) => Ok(n),
            v => Err(EvaluationError::Internal(format!(
                "Expected Num but got {:?}",
                v
            ))),
        }
    }
}
