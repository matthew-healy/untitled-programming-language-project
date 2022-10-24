use std::fmt::Debug;

#[derive(PartialEq, Eq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Unit,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(l, op, r) => write!(f, "({:?} {:?} {:?})", l, op, r),
            Expr::Unit => write!(f, "()"),
        }
    }
}

impl From<i32> for Expr {
    fn from(n: i32) -> Self {
        Expr::Number(n)
    }
}

#[derive(PartialEq, Eq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = match *self {
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Add => "+",
            Opcode::Sub => "-",
        };
        write!(f, "{}", w)
    }
}
