use std::{mem, cell::RefCell};

use crate::{ast::BinaryOp, env::Env, error::EvaluationError, values::Val};

mod compiler;
pub mod stack;

use self::stack::Stack;
pub use compiler::Compiler;

/// operations run by the vm.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Access(usize),
    Apply(),
    Binary(BinaryOp),
    Closure(Stack<Op>),
    Const(Val),
    Dummy(),
    EndLet(),
    Grab(),
    Join(),
    PushRetAddr(Stack<Op>),
    Return(),
    Sel(Stack<Op>, Stack<Op>),
    Update(),
}

pub struct VirtualMachine {
    code: Stack<Op>,
    env: Env<RefCell<Val>>,
    stack: Stack<Marker>,
}

impl VirtualMachine {
    pub fn new(code: Stack<Op>) -> Self {
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
                    self.stack.push(Marker::Val(v.into_inner()));
                }
                Op::Apply() => {
                    let (fn_body, fn_env) = self.stack.force_pop_closure()?;
                    self.code = fn_body;
                    self.env = fn_env;
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
                Op::Closure(body) => {
                    self.stack.push(Marker::Val(Val::Closure {
                        body,
                        env: self.env.clone(),
                    }))
                },
                Op::Const(v) => {
                    self.stack.push(Marker::Val(v))
                },
                Op::Dummy() => {
                    self.env.bind(RefCell::new(Val::Dummy))
                },
                Op::EndLet() => {
                    self.env.unbind()
                },
                Op::Return() => {
                    if self.stack.peek_closure().is_some() {
                        let (fn_body, fn_env) = self.stack.force_pop_closure()?;
                        self.code = fn_body;
                        self.env = fn_env;
                    } else {
                        let v = self.stack.force_pop_val()?;
                        self.stack.force_pop_app_delim()?;
                        let code = self.stack.force_pop_code()?;
                        let env = self.stack.force_pop_env()?;

                        self.code = code;
                        self.env = env;

                        self.stack.push(Marker::Val(v));
                    }
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
                Op::Grab() => {
                    if self.stack.peek_value().is_some() {
                        let val = self.stack.force_pop_val()?;
                        self.env.bind(RefCell::new(val));
                    } else {
                        self.stack.force_pop_app_delim()?;
                        let code = self.stack.force_pop_code()?;
                        let env = self.stack.force_pop_env()?;

                        let mut old_code = mem::replace(&mut self.code, code);
                        let old_env = mem::replace(&mut self.env, env);

                        old_code.push(Op::Grab());

                        self.stack.push(Marker::Val(Val::Closure { body: old_code, env: old_env }))
                    }
                },
                Op::PushRetAddr(c) => {
                    self.stack.push(Marker::Env(self.env.clone()));
                    self.stack.push(Marker::Code(c));
                    self.stack.push(Marker::AppDelim);
                },
                Op::Update() => {
                    let val = self.stack.force_pop_val()?;
                    self.env.update_first_match(val, |v| matches!(v, Val::Dummy));
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
    AppDelim,
    Code(Stack<Op>),
    Env(Env<RefCell<Val>>),
    Val(Val),
}

impl Stack<Marker> {
    fn force_pop(&mut self) -> Result<Marker, EvaluationError> {
        match self.pop() {
            Some(m) => {
                Ok(m)
            }
            None => Err(EvaluationError::Internal(String::from(
                "Attempt to pop from empty stack",
            ))),
        }
    }

    fn force_pop_app_delim(&mut self) -> Result<(), EvaluationError> {
        match self.force_pop()? {
            Marker::AppDelim => {
                Ok(())
            }
            _ => {
                Err(EvaluationError::Internal(format!(
                    "Expected AppDelim but got something else",
                    // m
                )))
            }
        }
    }

    fn force_pop_code(&mut self) -> Result<Stack<Op>, EvaluationError> {
        match self.force_pop()? {
            Marker::Code(c) => Ok(c),
            m => Err(EvaluationError::Internal(format!(
                "Expected Code but got {:?}",
                m
            ))),
        }
    }

    fn force_pop_env(&mut self) -> Result<Env<RefCell<Val>>, EvaluationError> {
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

    fn force_pop_closure(&mut self) -> Result<(Stack<Op>, Env<RefCell<Val>>), EvaluationError> {
        match self.force_pop_val()? {
            Val::Closure { body, env } => Ok((body, env)),
            m => Err(EvaluationError::Internal(format!(
                "Expected closure but got {:?}",
                m
            ))),
        }
    }

    fn peek_closure(&self) -> Option<(&Stack<Op>, &Env<RefCell<Val>>)> {
        match self.peek_value() {
            Some(Val::Closure { body, env }) => Some((body, env)),
            _ => None,
        }
    }

    fn peek_value(&self) -> Option<&Val> {
        match self.peek() {
            Some(Marker::Val(m)) => Some(m),
            _ => None
        }
    }
}