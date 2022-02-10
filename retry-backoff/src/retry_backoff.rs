use core::time::Duration;

pub trait RetryBackoff {
    /// attempts start from 1
    fn delay(&self, attempts: usize) -> Duration;
}
