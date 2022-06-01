use crate::error::{BareError, SourcedError};

use derive_more::From;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, From)]
pub struct ErrorSet<T>(Vec<T>);

pub type Errors = ErrorSet<SourcedError>;

impl Errors {
    pub fn attach_source(path: Option<PathBuf>, src: &str, bares: Vec<BareError>) -> Self {
        ErrorSet(
            bares
                .into_iter()
                .map(|bare| SourcedError::new(path.clone(), src, bare))
                .collect(),
        )
    }
}

impl<T> ErrorSet<T> {
    pub fn push(&mut self, error: T) {
        self.0.push(error)
    }

    pub fn extend(&mut self, sub: Self) {
        self.0.extend(sub.0);
    }

    pub fn into_result(self) -> Result<(), Self> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl<T> Default for ErrorSet<T> {
    fn default() -> Self {
        ErrorSet(vec![])
    }
}

impl<T> fmt::Display for ErrorSet<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in self.0.iter() {
            error.fmt(f)?;
        }
        Ok(())
    }
}
