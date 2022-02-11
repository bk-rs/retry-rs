use core::{fmt, ops::ControlFlow, time::Duration};

use retry_backoff::RetryBackoff;
use retry_predicate::RetryPredicate;

//
pub trait RetryPolicy<PParams> {
    fn backoff(&self) -> &dyn RetryBackoff;
    fn predicate(&self) -> &dyn RetryPredicate<PParams>;
    fn max_retries(&self) -> usize;

    fn next_step(&self, params: &PParams, attempts: usize) -> ControlFlow<StopReason, Duration> {
        if attempts > self.max_retries() {
            return ControlFlow::Break(StopReason::MaxRetriesReached);
        }

        if !self.predicate().test(params) {
            return ControlFlow::Break(StopReason::PredicateIsNotAllowed);
        }

        ControlFlow::Continue(self.backoff().delay(attempts))
    }

    fn name(&self) -> &str {
        "_"
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
