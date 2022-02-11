use core::fmt;

use retry_policy::StopReason as RetryPolicyStopReason;

//
pub struct Error<T> {
    pub stop_reason: RetryPolicyStopReason,
    errors: Vec<T>,
}

impl<T> Error<T> {
    pub(crate) fn new(stop_reason: RetryPolicyStopReason, errors: Vec<T>) -> Self {
        assert!(!errors.is_empty());

        Self {
            stop_reason,
            errors,
        }
    }

    pub fn last_error(mut self) -> T {
        self.errors.pop().expect("unreachable!()")
    }

    pub fn errors(self) -> Vec<T> {
        self.errors
    }
}

impl<T> fmt::Debug for Error<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("stop_reason", &self.stop_reason)
            .field("errors", &self.errors)
            .finish()
    }
}

impl<T> fmt::Display for Error<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> std::error::Error for Error<T> where T: fmt::Debug {}
