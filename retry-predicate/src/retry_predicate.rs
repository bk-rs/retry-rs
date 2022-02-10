use core::fmt;

//
pub trait RetryPredicate<Params> {
    /// returns true if a retry; false otherwise
    fn require_retry(&self, params: &Params) -> bool;

    fn name(&self) -> &str {
        "_"
    }
}

//
impl<Params> fmt::Debug for dyn RetryPredicate<Params> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPredicate")
            .field(&RetryPredicate::name(self))
            .finish()
    }
}

impl<Params> fmt::Debug for dyn RetryPredicate<Params> + Send {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPredicate")
            .field(&RetryPredicate::name(self))
            .finish()
    }
}

impl<Params> fmt::Debug for dyn RetryPredicate<Params> + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPredicate")
            .field(&RetryPredicate::name(self))
            .finish()
    }
}
