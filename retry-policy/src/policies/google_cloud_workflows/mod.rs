//! [Retry steps](https://cloud.google.com/workflows/docs/reference/syntax/retrying)
//!

use retry_backoff::backoffs::google_cloud_workflows::Backoff;
use retry_predicate::RetryPredicate;

pub mod http;

//
#[derive(Debug)]
pub struct Policy<E> {
    pub predicate: Box<dyn RetryPredicate<E> + Send + Sync>,
    pub max_retries: usize,
    pub backoff: Backoff,
}

impl<E> Policy<E> {
    pub fn new<P>(predicate: P, max_retries: usize, backoff: Backoff) -> Self
    where
        P: RetryPredicate<E> + Send + Sync + 'static,
    {
        Self {
            predicate: Box::new(predicate),
            max_retries,
            backoff,
        }
    }
}
