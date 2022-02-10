//! [Ref Function: retry.never](https://cloud.google.com/workflows/docs/reference/stdlib/retry/never)

use core::{fmt, marker::PhantomData};

use super::Predicate;

//
pub struct NeverPredicate<E> {
    phantom: PhantomData<E>,
}

impl<E> fmt::Debug for NeverPredicate<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NeverPredicate").finish()
    }
}

impl<E> Clone for NeverPredicate<E> {
    fn clone(&self) -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<E> Default for NeverPredicate<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> NeverPredicate<E> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

//
impl<E> Predicate<E> for NeverPredicate<E> {
    fn require_retry(&self, _err: &E) -> bool {
        false
    }

    fn name(&self) -> &str {
        "Never"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_predicate() {
        assert!(!Predicate::require_retry(&NeverPredicate::new(), &()));
        assert_eq!(Predicate::<()>::name(&NeverPredicate::new()), "Never");
    }
}
