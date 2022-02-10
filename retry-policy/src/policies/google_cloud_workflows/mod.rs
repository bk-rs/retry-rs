//! [Retry steps](https://cloud.google.com/workflows/docs/reference/syntax/retrying)
//!

use alloc::boxed::Box;

use retry_backoff::backoffs::google_cloud_workflows::Backoff;
use retry_predicate::RetryPredicate;

pub mod http;

//
#[derive(Debug)]
pub struct Policy<PParams> {
    pub predicate: Box<dyn RetryPredicate<PParams> + Send + Sync>,
    pub max_retries: usize,
    pub backoff: Backoff,
}

impl<PParams> Policy<PParams> {
    pub fn new<P>(predicate: P, max_retries: usize, backoff: Backoff) -> Self
    where
        P: RetryPredicate<PParams> + Send + Sync + 'static,
    {
        Self {
            predicate: Box::new(predicate),
            max_retries,
            backoff,
        }
    }
}

#[cfg(feature = "std")]
impl<PParams> crate::retry_policy::RetryPolicy<PParams> for Policy<PParams> {
    fn backoff(&self) -> &dyn retry_backoff::RetryBackoff {
        &self.backoff
    }
    fn predicate(&self) -> &dyn RetryPredicate<PParams> {
        self.predicate.as_ref()
    }
    fn max_retries(&self) -> usize {
        self.max_retries
    }

    fn name(&self) -> &str {
        "GoogleCloudWorkflows"
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_impl_retry_policy() {
        use core::{ops::ControlFlow, time::Duration};

        use retry_predicate::predicates::AlwaysPredicate;

        use crate::retry_policy::RetryPolicy;

        let policy = Policy::new(AlwaysPredicate, 1, Backoff::default());

        assert_eq!(
            RetryPolicy::retry(&policy, &(), 1),
            ControlFlow::Continue(Duration::from_secs(1))
        );
        assert_eq!(RetryPolicy::name(&policy), "GoogleCloudWorkflows");
    }
}
