use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Ident {
    s: String,
}

impl<'a> From<&'a str> for Ident {
    fn from(s: &'a str) -> Self {
        Ident { s: s.into() }
    }
}

#[derive(PartialEq, Eq)]
pub enum Expr {
    Let(Ident, Box<Expr>, Box<Expr>),
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Unit,
    Var(Ident),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Let(i, expr, body) => write!(f, "let {:?} = {:?} in {:?}", i, expr, body),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(l, op, r) => write!(f, "({:?} {:?} {:?})", l, op, r),
            Expr::Unit => write!(f, "()"),
            Expr::Var(i) => write!(f, "{:?}", i),
        }
    }
}

impl From<i32> for Expr {
    fn from(n: i32) -> Self {
        Expr::Number(n)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
