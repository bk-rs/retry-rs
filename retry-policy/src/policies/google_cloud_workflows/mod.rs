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

        use crate::retry_policy::{RetryPolicy, StopReason};

        //
        let policy = Policy::new(AlwaysPredicate, 1, Backoff::default());

        assert_eq!(
            RetryPolicy::next_step(&policy, &(), 1),
            ControlFlow::Continue(Duration::from_secs(1))
        );
        assert_eq!(RetryPolicy::name(&policy), "GoogleCloudWorkflows");

        //
        // Ref https://cloud.google.com/workflows/docs/reference/syntax/retrying#try-retry
        let policy = Policy::new(AlwaysPredicate, 8, Backoff::new(1.0, 60.0, 2.0));

        for (attempts, flow) in &[
            (1, ControlFlow::Continue(Duration::from_secs(1))),
            (2, ControlFlow::Continue(Duration::from_secs(2))),
            (3, ControlFlow::Continue(Duration::from_secs(4))),
            (4, ControlFlow::Continue(Duration::from_secs(8))),
            (5, ControlFlow::Continue(Duration::from_secs(16))),
            (6, ControlFlow::Continue(Duration::from_secs(32))),
            (7, ControlFlow::Continue(Duration::from_secs(60))),
            (8, ControlFlow::Continue(Duration::from_secs(60))),
            (9, ControlFlow::Break(StopReason::MaxRetriesReached)),
            (10, ControlFlow::Break(StopReason::MaxRetriesReached)),
        ] {
            assert_eq!(RetryPolicy::next_step(&policy, &(), *attempts), *flow);
        }
    }
}
