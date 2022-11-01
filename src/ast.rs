use std::fmt::Debug;

use crate::values::Val;

#[derive(PartialEq)]
pub enum RawExpr {
    Let(RawIdent, Box<RawExpr>, Box<RawExpr>),
    Literal(Val),
    Var(RawIdent),
    Op(Box<RawExpr>, Opcode, Box<RawExpr>),
}

impl Debug for RawExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawExpr::Let(i, bnd, body) => write!(f, "let {:?} = {:?} in {:?}", i, bnd, body),
            RawExpr::Literal(v) => write!(f, "{}", v),
            RawExpr::Op(l, op, r) => write!(f, "({:?} {:?} {:?})", l, op, r),
            RawExpr::Var(i) => write!(f, "{:?}", i),
        }
    }
}

#[derive(PartialEq)]
pub enum Expr {
    Let(Box<Expr>, Box<Expr>),
    Literal(Val),
    Var(usize),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Let(bnd, body) => write!(f, "let {:?} in {:?}", bnd, body),
            Expr::Literal(v) => write!(f, "{}", v),
            Expr::Op(l, op, r) => write!(f, "({:?} {:?} {:?})", l, op, r),
            Expr::Var(i) => write!(f, "{:?}", i),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RawIdent {
    s: String,
}

impl<'a> From<&'a str> for RawIdent {
    fn from(s: &'a str) -> Self {
        RawIdent { s: s.into() }
    }
}

impl From<f64> for RawExpr {
    fn from(n: f64) -> Self {
        RawExpr::Literal(Val::Num(n))
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
