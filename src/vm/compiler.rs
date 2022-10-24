use crate::{
    ast::{Expr, Opcode},
    values::Val,
};

/// operations run by the vm.
pub enum Op {
    Binary(Opcode),
    Const(Val),
}

pub struct Compiler {
    code: Vec<Op>,
}

impl Compiler {
    pub fn new() -> Self {
        let code = Vec::new();
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
            Expr::Number(n) => self.code.push(Op::Const(Val::Num(*n))),
            Expr::Op(l, op, r) => {
                self.code.push(Op::Binary(*op));
                self.push(r);
                self.push(l);
            }
            Expr::Unit => self.code.push(Op::Const(Val::Unit)),
        }
    }
}
