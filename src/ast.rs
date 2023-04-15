use std::{fmt::Debug, ops::Deref};

use crate::{types::Type, values::Val};

#[derive(PartialEq)]
pub enum RawExpr {
    App(Box<RawExpr>, Vec<RawExpr>),
    Lambda(Vec<(RawIdent, Type)>, Box<RawExpr>),
    Let(bool, RawIdent, Box<RawExpr>, Box<RawExpr>),
    Literal(Val),
    IfThenElse(Box<RawExpr>, Box<RawExpr>, Box<RawExpr>),
    Var(RawIdent),
    Op(Box<RawExpr>, BinaryOp, Box<RawExpr>),
}

impl Debug for RawExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawExpr::App(fnc, a) => write!(f, "({fnc:?} {a:?})"),
            RawExpr::Lambda(bindings, body) => {
                let bs = bindings
                    .iter()
                    .map(|(id, ty)| format!("{:?}: {:?}", id, ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "(|{bs:?}| {body:?})")
            }
            RawExpr::Let(rec, i, bnd, body) => {
                let rec_txt = if *rec { "rec " } else { "" };
                write!(f, "(let {rec_txt}{i:?} = {bnd:?} in {body:?})")
            }
            RawExpr::Literal(v) => write!(f, "{v}"),
            RawExpr::IfThenElse(cond, then, els) => {
                write!(f, "if {cond:?} then {then:?} else {els:?}")
            }
            RawExpr::Op(l, op, r) => write!(f, "({l:?} {op:?} {r:?})"),
            RawExpr::Var(i) => write!(f, "{i:?}"),
        }
    }
}

#[derive(PartialEq)]
pub enum Expr {
    App(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Type>, Box<Expr>),
    Let(bool, Box<Expr>, Box<Expr>),
    Literal(Val),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Var(usize),
    Op(Box<Expr>, BinaryOp, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::App(fnc, a) => write!(f, "{fnc:?} {a:?}"),
            Expr::Lambda(tys, body) => {
                let tys = tys
                    .iter()
                    .map(|ty| format!("{:?}", ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "|{tys:?}| {body:?}")
            }
            Expr::Let(rec, bnd, body) => {
                let rec = if *rec { "rec " } else { "" };
                write!(f, "let {rec}{bnd:?} in {body:?}")
            }
            Expr::Literal(v) => write!(f, "{v}"),
            Expr::IfThenElse(cond, thn, els) => write!(f, "if {cond:?} then {thn:?} else {els:?}"),
            Expr::Op(l, op, r) => write!(f, "({l:?} {op:?} {r:?})"),
            Expr::Var(i) => write!(f, "e[{i:?}]"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RawIdent {
    s: String,
}

impl Deref for RawIdent {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl Debug for RawIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
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
pub enum BinaryOp {
    Mul,
    Div,
    Add,
    Sub,
    Eq,
    And,
}

impl Debug for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = match *self {
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Eq => "==",
            BinaryOp::And => "&&",
        };
        write!(f, "{}", w)
    }
}
