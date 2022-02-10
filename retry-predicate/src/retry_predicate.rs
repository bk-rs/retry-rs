use core::fmt;

//
pub trait RetryPredicate<Params> {
    /// returns true if a retry; false otherwise
    /// [Ref](https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/function/Predicate.html#test(T))
    fn test(&self, params: &Params) -> bool;

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
