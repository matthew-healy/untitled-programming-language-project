use crate::ast::Expr;

use super::Op;

pub struct Compiler {
    code: Vec<Op>,
}

impl Compiler {
    pub fn new() -> Self {
        let code = Vec::new();
        Compiler { code }
    }

    fn for_closure() -> Self {
        let mut code = Vec::new();
        code.push(Op::Return());
        Compiler { code }
    }

    fn for_branch() -> Self {
        let mut code = Vec::new();
        code.push(Op::Join());
        Compiler { code }
    }

    /// compiles the syntax tree to a "bytecode" representation.
    ///
    /// note that instructions are returned in reverse order - i.e. the last
    /// element of the vector is the first instruction to be computed. this is
    /// because most of the time we just pop the next instruction off, and
    /// popping from the tail of a vector is O(1).
    pub fn compile(mut self, e: &Expr) -> Vec<Op> {
        self.push(e);
        self.code
    }

    fn push(&mut self, e: &Expr) {
        match e {
            Expr::App(fnc, a) => {
                self.code.push(Op::Apply());
                self.push(a);
                self.push(fnc);
            }
            Expr::Lambda(_, body) => {
                let body_ops = Compiler::for_closure().compile(body);
                self.code.push(Op::Closure(body_ops));
            }
            Expr::Let(binding, body) => {
                self.code.push(Op::EndLet());
                self.push(body);
                self.code.push(Op::Let());
                self.push(binding);
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
}
