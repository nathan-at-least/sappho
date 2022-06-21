mod application;
mod effects;
mod letexpr;
mod listform;
mod literal;
mod lookup;
mod matchexpr;
mod object;

use crate::{Eval, Result};
use sappho_east::GenExpr;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for GenExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use GenExpr::*;

        match &self {
            Lit(x) => x.eval(scope),
            Ref(x) => {
                let v = scope.deref(x)?;
                Ok(v)
            }
            Object(x) => x.eval(scope),
            List(x) => x.eval(scope),
            Let(x) => x.eval(scope),
            Match(x) => x.eval(scope),
            Application(x) => x.eval(scope),
            Lookup(x) => x.eval(scope),
            Effect(x) => x.eval(scope),
        }
    }
}