//! [Ref Function: retry.never](https://cloud.google.com/workflows/docs/reference/stdlib/retry/never)

use crate::retry_predicate::RetryPredicate;

//
#[derive(Debug, Clone, Default)]
pub struct Predicate;

//
impl<Params> RetryPredicate<Params> for Predicate {
    fn test(&self, _params: &Params) -> bool {
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
    fn test_impl_retry_predicate() {
        assert!(!RetryPredicate::test(&Predicate, &()));
        assert_eq!(RetryPredicate::<()>::name(&Predicate), "Never");
    }
}
