#![allow(refining_impl_trait)]

pub(crate) trait ResultHelper<T, E> {
    #[allow(dead_code)]
    fn replace<U>(self, ok: U) -> impl ResultHelper<U, E>;
    fn replace_err<F>(self, err: F) -> impl ResultHelper<T, F>;
}

impl<T, E> ResultHelper<T, E> for Result<T, E> {
    #[inline]
    fn replace<U>(self, val: U) -> impl ResultHelper<U, E> {
        match self {
            Ok(_) => Ok(val),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn replace_err<F>(self, err: F) -> Result<T, F> {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(err),
        }
    }
}
