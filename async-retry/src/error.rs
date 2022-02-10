use core::fmt;

pub enum Error<T> {
    Original(T),
    MaxRetriesReached,
}

impl<T> fmt::Debug for Error<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> fmt::Display for Error<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> std::error::Error for Error<T> where T: fmt::Debug {}
