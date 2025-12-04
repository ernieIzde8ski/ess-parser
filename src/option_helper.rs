#![allow(refining_impl_trait)]

pub(crate) trait OptionHelper<T> {
    fn or_err<E>(self, err: E) -> Result<T, E>;
}

impl<T> OptionHelper<T> for Option<T> {
    fn or_err<E>(self, err: E) -> Result<T, E> {
        match self {
            Some(val) => Ok(val),
            None => Err(err),
        }
    }
}
