use alloc::boxed::Box;
use core::{fmt, time::Duration};

use crate::retry_backoff::RetryBackoff;

//
pub struct Backoff {
    f: Box<dyn Fn(usize) -> Duration + Send + Sync>,
}

impl fmt::Debug for Backoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Backoff").finish_non_exhaustive()
    }
}

impl<F> From<F> for Backoff
where
    F: Fn(usize) -> Duration + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self { f: Box::new(f) }
    }
}

//
impl RetryBackoff for Backoff {
    fn delay(&self, attempts: usize) -> Duration {
        (self.f)(attempts)
    }

    fn name(&self) -> &str {
        "Fn"
    }
}

//
#[cfg(test)]
fn fn_demo(_attempts: usize) -> Duration {
    Duration::from_secs(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f() {
        let _ = Backoff::from(fn_demo);
        let _ = Backoff::from(|_attempts| Duration::from_secs(1));
    }

    #[test]
    fn test_impl_retry_backoff() {
        assert_eq!(
            RetryBackoff::delay(&Backoff::from(|_attempts| Duration::from_secs(1)), 1),
            Duration::from_secs(1)
        );
        assert_eq!(
            RetryBackoff::name(&Backoff::from(|_attempts| Duration::from_secs(1))),
            "Fn"
        );
    }
}
