use alloc::boxed::Box;

use retry_backoff::RetryBackoff;
use retry_predicate::RetryPredicate;

use crate::retry_policy::RetryPolicy;

//
#[derive(Debug)]
pub struct Policy<PParams> {
    pub predicate: Box<dyn RetryPredicate<PParams> + Send + Sync>,
    pub max_retries: usize,
    pub backoff: Box<dyn RetryBackoff + Send + Sync>,
}

impl<PParams> Policy<PParams> {
    pub fn new<P, BO>(predicate: P, max_retries: usize, backoff: BO) -> Self
    where
        P: RetryPredicate<PParams> + Send + Sync + 'static,
        BO: RetryBackoff + Send + Sync + 'static,
    {
        Self {
            predicate: Box::new(predicate),
            max_retries,
            backoff: Box::new(backoff),
        }
    }
}

impl<PParams> RetryPolicy<PParams> for Policy<PParams> {
    fn backoff(&self) -> &dyn RetryBackoff {
        self.backoff.as_ref()
    }
    fn predicate(&self) -> &dyn RetryPredicate<PParams> {
        self.predicate.as_ref()
    }
    fn max_retries(&self) -> usize {
        self.max_retries
    }

    fn name(&self) -> &str {
        "Simple"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::{ops::ControlFlow, time::Duration};

    use retry_backoff::backoffs::FnBackoff;
    use retry_predicate::predicates::AlwaysPredicate;

    #[test]
    fn test_impl_retry_policy() {
        let policy = Policy::new(
            AlwaysPredicate,
            1,
            FnBackoff::from(|_attempts: usize| Duration::from_secs(1)),
        );

        assert_eq!(
            RetryPolicy::retry(&policy, &(), 1),
            ControlFlow::Continue(Duration::from_secs(1))
        );
        assert_eq!(RetryPolicy::name(&policy), "Simple");
    }
}
