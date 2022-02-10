//! [Ref Function: retry.always](https://cloud.google.com/workflows/docs/reference/stdlib/retry/always)

use core::{fmt, marker::PhantomData};

use super::Predicate;

//
pub struct AlwaysPredicate<E> {
    phantom: PhantomData<E>,
}

impl<E> fmt::Debug for AlwaysPredicate<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AlwaysPredicate").finish()
    }
}

impl<E> Clone for AlwaysPredicate<E> {
    fn clone(&self) -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<E> Default for AlwaysPredicate<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> AlwaysPredicate<E> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

//
impl<E> Predicate<E> for AlwaysPredicate<E> {
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
    fn test_impl_predicate() {
        assert!(Predicate::require_retry(&AlwaysPredicate::new(), &()));
        assert_eq!(Predicate::<()>::name(&AlwaysPredicate::new()), "Always");
    }
}
