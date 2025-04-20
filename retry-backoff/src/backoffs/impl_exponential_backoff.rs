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
        self.iter()
            .nth(attempts)
            .unwrap_or_else(|| Some(*self.max()))
            .unwrap_or_else(|| *self.max())
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
        backoff.set_max(Duration::from_secs(1));

        assert!(RetryBackoff::delay(&backoff, 1) > Duration::from_millis(100));
        assert!(RetryBackoff::delay(&backoff, 2) > Duration::from_millis(200));
        assert!(RetryBackoff::delay(&backoff, 8) > Duration::from_millis(800));
        assert!(RetryBackoff::delay(&backoff, 9) > Duration::from_millis(900));
        assert_eq!(
            RetryBackoff::delay(&backoff, 10),
            Duration::from_millis(1000)
        );
        assert_eq!(
            RetryBackoff::delay(&backoff, 11),
            Duration::from_millis(1000)
        );
        assert_eq!(
            RetryBackoff::delay(&backoff, 100),
            Duration::from_millis(1000)
        );
        assert_eq!(RetryBackoff::name(&backoff), "CrateExponentialBackoff");
    }
}
