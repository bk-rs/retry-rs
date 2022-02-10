use alloc::boxed::Box;
use core::fmt;

use crate::retry_predicate::RetryPredicate;

//
pub struct Predicate<E> {
    f: Box<dyn Fn(&E) -> bool + Send + Sync>,
}

impl<E> fmt::Debug for Predicate<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FnPredicate").finish_non_exhaustive()
    }
}

impl<F, E> From<F> for Predicate<E>
where
    F: Fn(&E) -> bool + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self { f: Box::new(f) }
    }
}

//
impl<E> RetryPredicate<E> for Predicate<E> {
    fn require_retry(&self, err: &E) -> bool {
        (self.f)(err)
    }

    fn name(&self) -> &str {
        "Fn"
    }
}

//
#[cfg(test)]
fn fn_demo(_err: &usize) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f() {
        let _ = Predicate::from(fn_demo);
        let _ = Predicate::from(|_err: &usize| true);
    }

    #[test]
    fn test_impl_retry_predicate() {
        assert!(Predicate::require_retry(
            &Predicate::from(|_err: &usize| true),
            &0
        ));
        assert_eq!(
            Predicate::name(&Predicate::from(|_err: &usize| true),),
            "Fn"
        );
    }
}
