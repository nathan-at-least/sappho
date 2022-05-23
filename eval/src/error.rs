use crate::ValRef;
use derive_more::From;
use saplang_ast::Identifier;

#[derive(Debug, From)]
pub enum Error {
    Parse(saplang_parser::Errors),
    Unbound(Identifier),
    Uncallable(ValRef),
}

pub type Result<T> = std::result::Result<T, Error>;
