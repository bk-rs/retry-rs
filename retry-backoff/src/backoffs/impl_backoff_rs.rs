use core::time::Duration;

pub use backoff_rs::{Exponential as Backoff, ExponentialBackoffBuilder};

use crate::retry_backoff::RetryBackoff;

//
impl RetryBackoff for Backoff {
    fn delay(&self, attempts: usize) -> Duration {
        self.duration(attempts.saturating_sub(1))
    }

    fn name(&self) -> &str {
        "CrateBackoffRs"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_retry_backoff() {
        // Ref https://github.com/rust-playground/backoff-rs/blob/master/src/lib.rs#L116
        let backoff = ExponentialBackoffBuilder::default()
            .jitter(Duration::default())
            .max(Duration::from_secs(5))
            .build();

        assert_eq!(RetryBackoff::delay(&backoff, 1), Duration::from_millis(500));
        assert_eq!(RetryBackoff::name(&backoff), "CrateBackoffRs");
    }
}
