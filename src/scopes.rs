use crate::{
    ast::{Expr, RawExpr, RawIdent},
    error::{Error, ParseError},
};

pub struct ScopeChecker {
    idents: Vec<RawIdent>,
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
            RawExpr::App(fnc, a) => {
                let fnc = self.check(*fnc)?;
                let a = self.check(*a)?;
                Ok(Expr::App(Box::new(fnc), Box::new(a)))
            }
            RawExpr::Lambda(ident, ty, body) => {
                self.idents.push(ident);
                let body = Box::new(self.check(*body)?);
                self.idents.pop();
                Ok(Expr::Lambda(ty, body))
            }
            RawExpr::Let(ident, binding, body) => {
                let binding = Box::new(self.check(*binding)?);
                self.idents.push(ident);
                let body = Box::new(self.check(*body)?);
                Ok(Expr::Let(binding, body))
            }
            RawExpr::Literal(v) => Ok(Expr::Literal(v)),
            RawExpr::Op(l, op, r) => {
                let l = self.check(*l)?;
                let r = self.check(*r)?;
                Ok(Expr::Op(Box::new(l), op, Box::new(r)))
            }
            RawExpr::Var(raw_ident) => {
                let de_bruijn_idx = self
                    .idents
                    .iter()
                    .rev()
                    .position(|i| &raw_ident == i)
                    .ok_or_else(|| ParseError::UnboundIdentifier {
                        ident: raw_ident.clone(),
                    })?;
                Ok(Expr::Var(de_bruijn_idx))
            }
        }
    }
}
