use core::{fmt, time::Duration};

//
pub trait RetryBackoff {
    /// attempts start from 1
    fn delay(&self, attempts: usize) -> Duration;

    fn name(&self) -> &str {
        "_"
    }
}

impl fmt::Debug for dyn RetryBackoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryBackoff")
            .field(&RetryBackoff::name(self))
            .finish()
    }
}
impl fmt::Debug for dyn RetryBackoff + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryBackoff")
            .field(&RetryBackoff::name(self))
            .finish()
    }
}
