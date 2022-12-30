use std::mem;

use crate::{ast::BinaryOp, env::Env, error::EvaluationError, values::Val};

mod compiler;
mod stack;

use self::stack::Stack;
pub use compiler::Compiler;

/// operations run by the vm.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Access(usize),
    Apply(),
    Binary(BinaryOp),
    Closure(Vec<Op>),
    Const(Val),
    Let(),
    EndLet(),
    Return(),
    Sel(Vec<Op>, Vec<Op>),
    Join(),
}

pub struct VirtualMachine {
    code: Vec<Op>,
    env: Env<Val>,
    stack: Stack<Marker>,
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
                Op::Access(i) => {
                    let v = self.env.lookup(i).ok_or_else(|| EvaluationError::Internal(format!(
                        "Attempt to access unbound variable {:?}",
                        i
                    )))?;
                    self.stack.push(Marker::Val(v));
                }
                Op::Apply() => {
                    let arg = self.stack.force_pop_val()?;
                    let (fn_body, mut fn_env) = self.stack.force_pop_closure()?;
                    fn_env.bind(arg);

                    let prev_code = mem::replace(&mut self.code, fn_body);
                    let prev_env = self.env.clone();
                    self.env = fn_env;

                    self.stack.push(Marker::Env(prev_env));
                    self.stack.push(Marker::Code(prev_code));
                }
                Op::Binary(op) => {
                    let r = self.stack.force_pop_val()?;
                    let l = self.stack.force_pop_val()?;

                    let res = match op {
                        BinaryOp::Add => {
                            let (l, r) = (l.as_num()?, r.as_num()?);
                            Ok(Val::Num(l + r))
                        }
                        BinaryOp::Mul => {
                            let (l, r) = (l.as_num()?, r.as_num()?);
                            Ok(Val::Num(l * r))
                        }
                        BinaryOp::Sub => {
                            let (l, r) = (l.as_num()?, r.as_num()?);
                            Ok(Val::Num(l - r))
                        }
                        BinaryOp::Div => {
                            let (l, r) = (l.as_num()?, r.as_num()?);
                            if r == 0.0 {
                                Err(EvaluationError::DivisionByZero)
                            } else {
                                Ok(Val::Num(l / r))
                            }
                        }
                        BinaryOp::Eq => Ok(Val::Bool(l.try_eq(&r)?)),
                        BinaryOp::And => {
                            let (l, r) = (l.as_bool()?, r.as_bool()?);
                            Ok(Val::Bool(l && r))
                        }
                    }?;

                    self.stack.push(Marker::Val(res));
                }
                Op::Closure(body) => self.stack.push(Marker::Val(Val::Closure {
                    body,
                    env: self.env.clone(),
                })),
                Op::Const(v) => self.stack.push(Marker::Val(v)),
                Op::EndLet() => self.env.unbind(),
                Op::Let() => {
                    let v = self.stack.force_pop_val()?;
                    self.env.bind(v);
                }
                Op::Return() => {
                    let ret_val = self.stack.force_pop_val()?;

                    let code = self.stack.force_pop_code()?;
                    let env = self.stack.force_pop_env()?;

                    self.code = code;
                    self.env = env;

                    self.stack.push(Marker::Val(ret_val));
                }
                Op::Sel(thn, els) => {
                    let cond = self.stack.force_pop_bool()?;
                    let branch = if cond { thn } else { els };

                    let prev_code = mem::replace(&mut self.code, branch);

                    self.stack.push(Marker::Code(prev_code));
                }
                Op::Join() => {
                    let ret_val = self.stack.force_pop_val()?;
                    let code = self.stack.force_pop_code()?;

                    self.code = code;

                    self.stack.push(Marker::Val(ret_val));
                }
            }
        }

        if let Some(Marker::Val(v)) = self.stack.pop() {
            Ok(v)
        } else {
            Err(EvaluationError::Internal(
                "unexpected end of program".into(),
            ))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Marker {
    Code(Vec<Op>),
    Env(Env<Val>),
    Val(Val),
}

impl Stack<Marker> {
    fn force_pop(&mut self) -> Result<Marker, EvaluationError> {
        match self.pop() {
            Some(m) => Ok(m),
            None => Err(EvaluationError::Internal(String::from(
                "Attempt to pop from empty stack",
            ))),
        }
    }

    fn force_pop_code(&mut self) -> Result<Vec<Op>, EvaluationError> {
        match self.force_pop()? {
            Marker::Code(c) => Ok(c),
            m => Err(EvaluationError::Internal(format!(
                "Expected Code but got {:?}",
                m
            ))),
        }
    }

    fn force_pop_env(&mut self) -> Result<Env<Val>, EvaluationError> {
        match self.force_pop()? {
            Marker::Env(e) => Ok(e),
            m => Err(EvaluationError::Internal(format!(
                "Expected Env but got {:?}",
                m
            ))),
        }
    }

    fn force_pop_val(&mut self) -> Result<Val, EvaluationError> {
        match self.force_pop()? {
            Marker::Val(v) => Ok(v),
            m => Err(EvaluationError::Internal(format!(
                "Expected Val but got {:?}",
                m
            ))),
        }
    }

    fn force_pop_bool(&mut self) -> Result<bool, EvaluationError> {
        match self.force_pop_val()? {
            Val::Bool(b) => Ok(b),
            m => Err(EvaluationError::Internal(format!(
                "Expected bool but got {:?}",
                m
            ))),
        }
    }

    fn force_pop_closure(&mut self) -> Result<(Vec<Op>, Env<Val>), EvaluationError> {
        match self.force_pop_val()? {
            Val::Closure { body, env } => Ok((body, env)),
            m => Err(EvaluationError::Internal(format!(
                "Expected closure but got {:?}",
                m
            ))),
        }
    }
}
