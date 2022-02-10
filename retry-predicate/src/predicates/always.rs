//! [Ref Function: retry.always](https://cloud.google.com/workflows/docs/reference/stdlib/retry/always)

use core::{fmt, marker::PhantomData};

use crate::retry_predicate::RetryPredicate;

//
pub struct Predicate<E> {
    phantom: PhantomData<E>,
}

impl<E> fmt::Debug for Predicate<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AlwaysPredicate").finish()
    }
}

impl<E> Clone for Predicate<E> {
    fn clone(&self) -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<E> Default for Predicate<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Predicate<E> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

//
impl<E> RetryPredicate<E> for Predicate<E> {
    fn require_retry(&self, _err: &E) -> bool {
        true
    }

    fn name(&self) -> &str {
        "Always"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_retry_predicate() {
        assert!(RetryPredicate::require_retry(&Predicate::new(), &()));
        assert_eq!(RetryPredicate::<()>::name(&Predicate::new()), "Always");
    }
}
