use core::fmt;

//
pub trait RetryPredicate<E> {
    /// returns true if a retry; false otherwise
    fn require_retry(&self, err: &E) -> bool;

    fn name(&self) -> &str {
        "_"
    }
}

//
impl<E> fmt::Debug for dyn RetryPredicate<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPredicate")
            .field(&RetryPredicate::name(self))
            .finish()
    }
}

impl<E> fmt::Debug for dyn RetryPredicate<E> + Send {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPredicate")
            .field(&RetryPredicate::name(self))
            .finish()
    }
}

impl<E> fmt::Debug for dyn RetryPredicate<E> + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPredicate")
            .field(&RetryPredicate::name(self))
            .finish()
    }
}
