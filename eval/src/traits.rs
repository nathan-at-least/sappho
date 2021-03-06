use crate::Result;
use sappho_value::{ScopeRef, ValRef, Value};

pub(crate) trait Eval {
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef>;
}

pub(crate) trait EvalV {
    fn eval_val(&self, scope: &ScopeRef) -> Result<Value>;
}

pub(crate) trait EvalThunk {
    fn eval_thunk(&self) -> Result<ValRef>;
}

impl<T> Eval for T
where
    T: EvalV,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        self.eval_val(scope).map(ValRef::from)
    }
}
