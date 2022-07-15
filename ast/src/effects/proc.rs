use crate::Expr;
use sappho_unparse::{Stream, Unparse};

/// Proc expressions can cause mutations (in memory or I/O), as in `!launch_balloon`, as well as
/// causing [QueryEffects](crate::QueryEffects).
pub type ProcExpr = Expr<ProcEffects>;

/// A proc effect can either be a mutation or a query effect.
#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    /// Inquire is identical to [QueryEffects::Inquire](crate::QueryEffects::Inquire).
    Inquire(Box<Expr<ProcEffects>>),

    /// Evoke a mutation, as in `!exit`.
    Evoke(Box<Expr<ProcEffects>>),
}

impl From<ProcEffects> for ProcExpr {
    fn from(x: ProcEffects) -> Self {
        Expr::Effect(x)
    }
}

impl Unparse for ProcEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use ProcEffects::*;

        match self {
            Inquire(x) => {
                s.write("$");
                s.write(x);
            }
            Evoke(x) => {
                s.write("!");
                s.write(x);
            }
        }
    }
}
