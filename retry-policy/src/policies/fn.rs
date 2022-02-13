use alloc::boxed::Box;
use core::{fmt, ops::ControlFlow, time::Duration};

use retry_backoff::RetryBackoff;
use retry_predicate::RetryPredicate;

use crate::retry_policy::{RetryPolicy, StopReason};

//
pub struct Policy<PParams> {
    f: Box<dyn Fn(&PParams, usize) -> ControlFlow<StopReason, Duration> + Send + Sync>,
}

impl<PParams> fmt::Debug for Policy<PParams> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FnPolicy").finish_non_exhaustive()
    }
}

impl<PParams, F> From<F> for Policy<PParams>
where
    F: Fn(&PParams, usize) -> ControlFlow<StopReason, Duration> + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self { f: Box::new(f) }
    }
}

//
impl<PParams> RetryPolicy<PParams> for Policy<PParams>
where
    PParams: Default,
{
    fn predicate(&self) -> &dyn RetryPredicate<PParams> {
        unreachable!()
    }
    fn max_retries(&self) -> usize {
        unreachable!()
    }
    fn backoff(&self) -> &dyn RetryBackoff {
        unreachable!()
    }

    fn next_step(&self, params: &PParams, attempts: usize) -> ControlFlow<StopReason, Duration> {
        (self.f)(params, attempts)
    }

    fn name(&self) -> &str {
        "Fn"
    }
}

//
#[cfg(test)]
fn fn_demo(_params: &(), _attempts: usize) -> ControlFlow<StopReason, Duration> {
    ControlFlow::Continue(Duration::from_secs(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f() {
        let _ = Policy::from(fn_demo);
        let _ = Policy::from(|_params: &(), _attempts: usize| {
            ControlFlow::Continue(Duration::from_secs(1))
        });
    }

    #[test]
    fn test_impl_retry_policy() {
        let policy = Policy::from(fn_demo);

        assert_eq!(
            RetryPolicy::next_step(&policy, &(), 1),
            ControlFlow::Continue(Duration::from_secs(1))
        );
        assert_eq!(RetryPolicy::name(&policy), "Fn");
    }
}
