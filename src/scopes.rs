use crate::{
    ast::{Expr, RawExpr},
    error::{Error, ParseError},
    interner,
};

pub struct ScopeChecker {
    idents: Vec<interner::Id>,
}

impl ScopeChecker {
    pub fn new() -> ScopeChecker {
        let idents = Vec::new();
        ScopeChecker { idents }
    }
}

impl ScopeChecker {
    pub fn check(&mut self, raw_expr: RawExpr) -> Result<Expr, Error> {
        match raw_expr {
            RawExpr::Ascribed(e, t) => Ok(Expr::Ascribed(Box::new(self.check(*e)?), t)),
            RawExpr::App(fnc, args) => {
                let fnc = self.check(*fnc)?;
                let args = args
                    .into_iter()
                    .map(|a| self.check(a))
                    .collect::<Result<_, _>>()?;
                Ok(Expr::App(Box::new(fnc), args))
            }
            RawExpr::Lambda(id, ty, body) => {
                self.idents.push(id);
                let body = Box::new(self.check(*body)?);
                self.idents.pop();
                Ok(Expr::Lambda(ty, body))
            }
            RawExpr::Let(false, ident, binding, body) => {
                let binding = Box::new(self.check(*binding)?);
                self.idents.push(ident);
                let body = Box::new(self.check(*body)?);
                Ok(Expr::Let(false, binding, body))
            }
            RawExpr::Let(true, ident, binding, body) => {
                self.idents.push(ident);
                let binding = Box::new(self.check(*binding)?);
                let body = Box::new(self.check(*body)?);
                Ok(Expr::Let(true, binding, body))
            }
            RawExpr::Literal(v) => Ok(Expr::Literal(v)),
            RawExpr::IfThenElse(cond, thn, els) => {
                let cond = Box::new(self.check(*cond)?);
                let thn = Box::new(self.check(*thn)?);
                let els = Box::new(self.check(*els)?);
                Ok(Expr::IfThenElse(cond, thn, els))
            }
            RawExpr::Op(l, op, r) => {
                let l = self.check(*l)?;
                let r = self.check(*r)?;
                Ok(Expr::Op(Box::new(l), op, Box::new(r)))
            }
            RawExpr::Var(id) => {
                let de_bruijn_idx = self
                    .idents
                    .iter()
                    .rev()
                    .position(|i| &id == i)
                    .ok_or(ParseError::UnboundIdentifier { ident: id })?;
                Ok(Expr::Var(de_bruijn_idx))
            }
        }
    }
}
