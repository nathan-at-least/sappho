use std::fmt;

/// A function definition expression, ie `fn x -> x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct FuncDef<Pattern, PureExpr> {
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: Box<PureExpr>,
}

impl<P, X> FuncDef<P, X> {
    pub fn transform_into<PD, XD>(self) -> FuncDef<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        FuncDef {
            binding: PD::from(self.binding),
            body: Box::new(XD::from(*self.body)),
        }
    }
}

impl<P, X> fmt::Display for FuncDef<P, X>
where
    P: fmt::Display,
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
