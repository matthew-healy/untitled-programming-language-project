use crate::ast::Expr;

use super::{Op, stack::Stack};

enum CompilerMode {
    Normal,
    Tail
}

// Compilation scheme inspired by ZAM
// see here: https://xavierleroy.org/mpri/2-4/machines.2up.pdf
pub struct Compiler {
    mode: CompilerMode,
    code: Stack<Op>,
}

impl Compiler {
    pub fn new() -> Self {
        let mode = CompilerMode::Normal;
        let code = Stack::new();
        Compiler { mode, code }
    }

    fn with_mode_and_ops<I: IntoIterator<Item=Op>>(mode: CompilerMode, i: I) -> Self {
        let code: Vec<Op> = i.into_iter().collect();
        let code = Stack::from_stacked_vec(code);
        Compiler { mode, code }
    }

    fn for_tail() -> Self {
        Self::with_mode_and_ops(CompilerMode::Tail, None)
    }

    fn for_branch() -> Self {
        Self::with_mode_and_ops(
            CompilerMode::Normal,
            Some(Op::Join())
        )
    }

    /// compiles the syntax tree to a "bytecode" representation.
    ///
    /// note that instructions are returned in reverse order - i.e. the last
    /// element of the vector is the first instruction to be computed. this is
    /// because most of the time we just pop the next instruction off, and
    /// popping from the tail of a vector is O(1).
    pub fn compile(mut self, e: &Expr) -> Stack<Op> {
        match self.mode {
            CompilerMode::Normal => self.push(e),
            CompilerMode::Tail => self.push_tail(e)
        }
        self.code
    }

    fn push(&mut self, e: &Expr) {
        match e {
            Expr::App(fnc, a) => {
                let code = std::mem::take(&mut self.code);
                self.code.push(Op::Apply());
                self.push(fnc);
                self.push(a);
                self.code.push(Op::PushRetAddr(code));
            }
            Expr::Lambda(_, body) => {
                let mut code = Compiler::for_tail().compile(body);
                code.push(Op::Grab());
                self.code.push(Op::Closure(code));
            }
            Expr::Let(false, binding, body) => {
                self.code.push(Op::EndLet());
                self.push(body);
                self.code.push(Op::Grab());
                self.push(binding);
            }
            Expr::Let(true, binding, body) => {
                self.code.push(Op::EndLet());
                self.push(body);
                self.code.push(Op::Update());
                self.push(binding);
                self.code.push(Op::Dummy());
            }
            Expr::Literal(v) => self.code.push(Op::Const(v.clone())),
            Expr::IfThenElse(cond, thn, els) => {
                let thn_ops = Compiler::for_branch().compile(thn);
                let els_ops = Compiler::for_branch().compile(els);
                self.code.push(Op::Sel(thn_ops, els_ops));
                self.push(cond);
            }
            Expr::Op(l, op, r) => {
                self.code.push(Op::Binary(*op));
                self.push(r);
                self.push(l);
            }
            Expr::Var(i) => {
                self.code.push(Op::Access(*i));
            }
        }
    }

    fn push_tail(&mut self, e: &Expr) {
        match e {
            Expr::App(a, b) => {
                self.push_tail(a);
                self.push(b);
            },
            Expr::Lambda(_, a) => {
                self.push_tail(a);
                self.code.push(Op::Grab());
            }
            Expr::Let(false, a, b) => {
                self.push_tail(b);
                self.code.push(Op::Grab());
                self.push(a);
            },
            Expr::Let(true, a, b) => {
                self.push_tail(b);
                self.code.push(Op::Update());
                self.push(a);
                self.code.push(Op::Dummy());
            }
            a => {
                self.code.push(Op::Return());
                self.push(a);
            }
        }
    }
}
