use alloc::boxed::Box;
use core::fmt;

use crate::retry_predicate::RetryPredicate;

//
pub struct Predicate<Params> {
    f: Box<dyn Fn(&Params) -> bool + Send + Sync>,
}

impl<Params> fmt::Debug for Predicate<Params> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FnPredicate").finish_non_exhaustive()
    }
}

impl<F, Params> From<F> for Predicate<Params>
where
    F: Fn(&Params) -> bool + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self { f: Box::new(f) }
    }
}

//
impl<Params> RetryPredicate<Params> for Predicate<Params> {
    fn require_retry(&self, params: &Params) -> bool {
        (self.f)(params)
    }

    fn name(&self) -> &str {
        "Fn"
    }
}

//
#[cfg(test)]
fn fn_demo(_params: &usize) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f() {
        let _ = Predicate::from(fn_demo);
        let _ = Predicate::from(|_params: &usize| true);
    }

    #[test]
    fn test_impl_retry_predicate() {
        let predicate = Predicate::from(fn_demo);

        assert!(Predicate::require_retry(&predicate, &0));
        assert_eq!(Predicate::name(&predicate), "Fn");
    }
}
