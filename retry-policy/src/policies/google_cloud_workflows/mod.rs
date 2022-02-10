//! [Retry steps](https://cloud.google.com/workflows/docs/reference/syntax/retrying)
//!

use core::fmt;

use retry_backoff::backoffs::google_cloud_workflows::Backoff;

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
