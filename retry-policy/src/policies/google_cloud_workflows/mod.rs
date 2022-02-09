//! [Retry steps](https://cloud.google.com/workflows/docs/reference/syntax/retrying)
//!

use core::fmt;

#[cfg(feature = "http")]
pub mod http;

//
pub struct Policy<E> {
    /// returns true if a retry; false otherwise
    pub predicate: Box<dyn Fn(&E) -> bool>,
    pub max_retries: usize,
    pub backoff: Backoff,
}

impl<E> fmt::Debug for Policy<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Policy")
            .field("predicate", &"")
            .field("max_retries", &self.max_retries)
            .field("backoff", &self.backoff)
            .finish()
    }
}

impl<E> Policy<E> {
    pub fn new<F>(predicate: F, max_retries: usize, backoff: Backoff) -> Self
    where
        F: Fn(&E) -> bool + 'static,
    {
        Self {
            predicate: Box::new(predicate),
            max_retries,
            backoff,
        }
    }
}

/// [Function: retry.always](https://cloud.google.com/workflows/docs/reference/stdlib/retry/always)
pub fn always_predicate<E>(_: &E) -> bool {
    true
}

/// [Function: retry.never](https://cloud.google.com/workflows/docs/reference/stdlib/retry/never)
pub fn never_predicate<E>(_: &E) -> bool {
    false
}

//
#[derive(Debug, PartialEq)]
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

impl Backoff {
    pub fn new(initial_delay_secs: f32, max_delay_secs: f32, multiplier: f32) -> Self {
        Self {
            initial_delay_secs,
            max_delay_secs,
            multiplier,
        }
    }
}

/// [Object: retry.default_backoff](https://cloud.google.com/workflows/docs/reference/stdlib/retry/default_backoff)
pub fn default_backoff() -> Backoff {
    Backoff::new(1.0, 60.0, 1.25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_predicate() {
        let policy = Policy::<usize>::new(always_predicate, 1, default_backoff());
        assert!((policy.predicate)(&0));
    }

    #[test]
    fn test_never_predicate() {
        let policy = Policy::<usize>::new(never_predicate, 1, default_backoff());
        assert!(!(policy.predicate)(&0));
    }

    #[test]
    fn test_default_backoff() {
        let backoff = default_backoff();
        assert_eq!(backoff.initial_delay_secs, 1.0);
        assert_eq!(backoff.max_delay_secs, 60.0);
        assert_eq!(backoff.multiplier, 1.25);
    }
}
