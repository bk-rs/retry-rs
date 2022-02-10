use core::time::Duration;

use crate::retry_backoff::RetryBackoff;

//
pub struct Backoff {
    f: Box<dyn Fn(usize) -> Duration + Send + Sync>,
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
    }
}