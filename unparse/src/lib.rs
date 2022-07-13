use std::fmt::Display;
pub use std::fmt::{Formatter, Result as FmtResult};

pub trait Unparse {
    fn unparse_into(&self, s: &mut Stream) {
}

pub fn indent(f: &mut Formatter, depth: usize) -> FmtResult {
    for _ in 0..depth {
        write!(f, "  ")?;
    }
    Ok(())
}

impl Unparse for String {
    fn unparse_into(&self, s: &mut Stream) {
        self.fmt(f)
    }
}

impl<X> Unparse for Box<X>
where
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.as_ref().unparse(f, depth)
    }
}
