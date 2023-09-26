use std::{fmt::Debug, ops::Deref};

use crate::{typ::Type, values::Val};

#[derive(PartialEq)]
pub enum RawExpr {
    Ascribed(Box<RawExpr>, Type),
    App(Box<RawExpr>, Vec<RawExpr>),
    Lambda(RawIdent, Type, Box<RawExpr>),
    Let(bool, RawIdent, Box<RawExpr>, Box<RawExpr>),
    Literal(Val),
    IfThenElse(Box<RawExpr>, Box<RawExpr>, Box<RawExpr>),
    Var(RawIdent),
    Op(Box<RawExpr>, BinaryOp, Box<RawExpr>),
}

impl RawExpr {
    pub fn lambda(args: Vec<(RawIdent, Type)>, body: Box<RawExpr>) -> Box<Self> {
        args.into_iter()
            .rev()
            .fold(body, |body, (id, ty)| Box::new(Self::Lambda(id, ty, body)))
    }
}

impl Debug for RawExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawExpr::Ascribed(e, t) => write!(f, "{e:?} : {t:?}"),
            RawExpr::App(fnc, a) => write!(f, "({fnc:?} {a:?})"),
            RawExpr::Lambda(id, ty, body) => write!(f, "|{id:?}: {ty:?}| {body:?}"),
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

#[derive(Clone, PartialEq)]
pub enum Expr {
    Ascribed(Box<Expr>, Type),
    App(Box<Expr>, Vec<Expr>),
    Lambda(Type, Box<Expr>),
    Let(bool, Box<Expr>, Box<Expr>),
    Literal(Val),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Var(usize),
    Op(Box<Expr>, BinaryOp, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Ascribed(e, t) => write!(f, "{e:?} : {t:?}"),
            Expr::App(fnc, a) => write!(f, "{fnc:?} {a:?}"),
            Expr::Lambda(ty, body) => write!(f, "|{ty:?}| {body:?}"),
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
