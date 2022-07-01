use crate::Pattern;
use std::fmt;

/// A function definition expression, ie `fn x -> x`.
#[derive(Debug, PartialEq, derive_new::new)]
pub struct FuncDef<PureExpr> {
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: Box<PureExpr>,
}

impl<X> fmt::Display for FuncDef<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn ")?;
        self.binding.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
