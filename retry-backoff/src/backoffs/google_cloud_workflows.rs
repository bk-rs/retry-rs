use core::{cmp::min, time::Duration};

use crate::retry_backoff::RetryBackoff;

//
#[derive(Debug, Clone, PartialEq)]
pub struct Backoff {
    pub initial_delay_secs: f32,
    pub max_delay_secs: f32,
    pub multiplier: f32,
}

impl Default for Backoff {
    fn default() -> Self {
        default_backoff()
    }
}

/// [Object: retry.default_backoff](https://cloud.google.com/workflows/docs/reference/stdlib/retry/default_backoff)
pub fn default_backoff() -> Backoff {
    Backoff::new(1.0, 60.0, 1.25)
}

impl Backoff {
    pub fn new(initial_delay_secs: f32, max_delay_secs: f32, multiplier: f32) -> Self {
        Self {
            initial_delay_secs,
            max_delay_secs,
            multiplier,
        }
    }

    pub fn delay(&self, attempts: usize) -> Duration {
        match attempts {
            0 => unreachable!(),
            1 => Duration::from_millis((self.initial_delay_secs * 1000.0).round() as u64),
            n => Duration::from_millis(min(
                (self.multiplier.powi((n - 1) as i32) * 1000.0).round() as u64,
                (self.max_delay_secs * 1000.0).round() as u64,
            )),
        }
    }
}

//
impl RetryBackoff for Backoff {
    fn delay(&self, attempts: usize) -> Duration {
        Self::delay(self, attempts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_backoff() {
        let backoff = default_backoff();
        assert_eq!(backoff.initial_delay_secs, 1.0);
        assert_eq!(backoff.max_delay_secs, 60.0);
        assert_eq!(backoff.multiplier, 1.25);

        assert_eq!(Backoff::default(), default_backoff());
    }

    #[test]
    fn test_delay() {
        let backoff = Backoff::new(1.0, 60.0, 2.0);
        for (attempts, secs) in &[
            (1, 1),
            (2, 2),
            (3, 4),
            (4, 8),
            (5, 16),
            (6, 32),
            (7, 60),
            (8, 60),
            (9, 60),
            (10, 60),
        ] {
            assert_eq!(backoff.delay(*attempts), Duration::from_secs(*secs));
            assert_eq!(
                RetryBackoff::delay(&backoff, *attempts),
                Duration::from_secs(*secs)
            );
        }
    }
}
