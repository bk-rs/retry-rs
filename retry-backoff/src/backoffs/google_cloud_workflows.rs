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
    fn test_default_backoff() {
        let backoff = default_backoff();
        assert_eq!(backoff.initial_delay_secs, 1.0);
        assert_eq!(backoff.max_delay_secs, 60.0);
        assert_eq!(backoff.multiplier, 1.25);

        assert_eq!(Backoff::default(), default_backoff());
    }
}
