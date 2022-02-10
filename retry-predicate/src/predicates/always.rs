//! [Ref Function: retry.always](https://cloud.google.com/workflows/docs/reference/stdlib/retry/always)

use crate::retry_predicate::RetryPredicate;

//
#[derive(Debug, Clone, Default)]
pub struct Predicate;

//
impl<Params> RetryPredicate<Params> for Predicate {
    fn require_retry(&self, _params: &Params) -> bool {
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
        assert!(RetryPredicate::require_retry(&Predicate, &()));
        assert_eq!(RetryPredicate::<()>::name(&Predicate), "Always");
    }
}
