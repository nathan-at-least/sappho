use crate::{Coerce, Result, Value};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ValRef(Rc<Value>);

impl ValRef {
    pub fn coerce<T>(&self) -> Result<&T>
    where
        T: Coerce,
    {
        use crate::Error::CoercionFailure;

        T::coerce_from_value(&self.0)
            .ok_or_else(|| CoercionFailure(self.clone(), std::any::type_name::<Self>()))
    }
}

impl Deref for ValRef {
    type Target = Value;

    fn deref(&self) -> &Value {
        self.0.deref()
    }
}

impl Borrow<Value> for ValRef {
    fn borrow(&self) -> &Value {
        self.0.borrow()
    }
}

impl<T> From<T> for ValRef
where
    Value: From<T>,
{
    fn from(v: T) -> Self {
        ValRef(Rc::new(Value::from(v)))
    }
}

impl fmt::Display for ValRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // BUG: Why doesn't `Deref` automatically enable `Display`?
        self.deref().fmt(f)
    }
}

#[cfg(test)]
impl ValRef {
    /// If Self is the only holder of the value, return it, otherwise panic.
    pub fn unwrap(self) -> Value {
        Rc::try_unwrap(self.0).unwrap()
    }
}