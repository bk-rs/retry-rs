use core::fmt;

use retry_policy::retry_policy::StopReason as RetryPolicyStopReason;

//
pub struct Error<T> {
    pub kind: ErrorKind,
    pub last: T,
}

impl<T> Error<T> {
    pub fn new(kind: ErrorKind, last: T) -> Self {
        Self { kind, last }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    Original,
    RetryPolicyStopReason(RetryPolicyStopReason),
}

impl<T> fmt::Debug for Error<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("last", &self.last)
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
