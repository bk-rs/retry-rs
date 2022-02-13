use core::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use exponential_backoff::Backoff as Inner;

use crate::retry_backoff::RetryBackoff;

//
#[derive(Debug, Clone)]
pub struct Backoff(Inner);

impl Deref for Backoff {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Backoff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//
impl Backoff {
    #[inline]
    pub fn new(min: Duration, max: impl Into<Option<Duration>>) -> Self {
        Self(Inner::new(u32::MAX, min, max))
    }
}

//
impl RetryBackoff for Backoff {
    fn delay(&self, attempts: usize) -> Duration {
        self.next(attempts as u32).unwrap_or_else(|| {
            self.next((attempts as u32).saturating_sub(1))
                .expect("unreachable!()")
        })
    }

    fn name(&self) -> &str {
        "CrateExponentialBackoff"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_retry_backoff() {
        let mut backoff = Backoff::new(Duration::from_millis(100), None);
        backoff.set_max(Some(Duration::from_secs(10)));

        assert!(RetryBackoff::delay(&backoff, 1) > Duration::from_millis(100));
        assert_eq!(RetryBackoff::name(&backoff), "CrateExponentialBackoff");
    }
}
