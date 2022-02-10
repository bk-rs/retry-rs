use core::{fmt, ops::ControlFlow, time::Duration};

use retry_backoff::RetryBackoff;
use retry_predicate::RetryPredicate;

//
pub trait RetryPolicy<PParams> {
    fn backoff(&self) -> &dyn RetryBackoff;
    fn predicate(&self) -> &dyn RetryPredicate<PParams>;
    fn max_retries(&self) -> usize;

    fn retry(&self, params: &PParams, attempts: usize) -> ControlFlow<StopReason, Duration> {
        if self.max_retries() > attempts {
            return ControlFlow::Break(StopReason::MaxRetriesReached);
        }

        if !self.predicate().require_retry(params) {
            return ControlFlow::Break(StopReason::PredicateIsNotAllowed);
        }

        ControlFlow::Continue(self.backoff().delay(attempts))
    }

    fn name(&self) -> &str {
        "_"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StopReason {
    MaxRetriesReached,
    PredicateIsNotAllowed,
}

//
impl<PParams> fmt::Debug for dyn RetryPolicy<PParams> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPolicy")
            .field(&RetryPolicy::name(self))
            .finish()
    }
}

impl<PParams> fmt::Debug for dyn RetryPolicy<PParams> + Send {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPolicy")
            .field(&RetryPolicy::name(self))
            .finish()
    }
}

impl<PParams> fmt::Debug for dyn RetryPolicy<PParams> + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RetryPolicy")
            .field(&RetryPolicy::name(self))
            .finish()
    }
}
