//! [Retry steps](https://cloud.google.com/workflows/docs/reference/syntax/retrying)
//!

use core::fmt;

use retry_backoff::backoffs::google_cloud_workflows::Backoff;

#[cfg(feature = "http")]
pub mod http;

//
pub struct Policy<E> {
    /// returns true if a retry; false otherwise
    pub predicate: Box<dyn Fn(&E) -> bool>,
    pub max_retries: usize,
    pub backoff: Backoff,
}

impl<E> fmt::Debug for Policy<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Policy")
            .field("predicate", &"")
            .field("max_retries", &self.max_retries)
            .field("backoff", &self.backoff)
            .finish()
    }
}

impl<E> Policy<E> {
    pub fn new<F>(predicate: F, max_retries: usize, backoff: Backoff) -> Self
    where
        F: Fn(&E) -> bool + 'static,
    {
        Self {
            predicate: Box::new(predicate),
            max_retries,
            backoff,
        }
    }
}

/// [Function: retry.always](https://cloud.google.com/workflows/docs/reference/stdlib/retry/always)
pub fn always_predicate<E>(_: &E) -> bool {
    true
}

/// [Function: retry.never](https://cloud.google.com/workflows/docs/reference/stdlib/retry/never)
pub fn never_predicate<E>(_: &E) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_predicate() {
        let policy = Policy::<usize>::new(always_predicate, 1, Backoff::default());
        assert!((policy.predicate)(&0));
    }

    #[test]
    fn test_never_predicate() {
        let policy = Policy::<usize>::new(never_predicate, 1, Backoff::default());
        assert!(!(policy.predicate)(&0));
    }
}
