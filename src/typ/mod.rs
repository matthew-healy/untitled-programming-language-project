use std::fmt::Display;

pub mod check;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    Bool,
    Num,
    Unit,
    UnificationVar(usize),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Arrow(t1, t2) => write!(f, "{t1} -> {t2}"),
            Type::Bool => write!(f, "Bool"),
            Type::Num => write!(f, "Num"),
            Type::Unit => write!(f, "Unit"),
            Type::UnificationVar(n) => write!(f, "?{n}"),
        }
    }
}
