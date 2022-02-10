use super::Predicate;

//
pub struct FnPredicate<E> {
    f: Box<dyn Fn(&E) -> bool + Send + Sync>,
}

impl<F, E> From<F> for FnPredicate<E>
where
    F: Fn(&E) -> bool + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self { f: Box::new(f) }
    }
}

//
impl<E> Predicate<E> for FnPredicate<E> {
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
        let _ = FnPredicate::from(fn_demo);
        let _ = FnPredicate::from(|_err: &usize| true);
    }

    #[test]
    fn test_impl_predicate() {
        assert!(Predicate::require_retry(
            &FnPredicate::from(|_err: &usize| true),
            &0
        ));
        assert_eq!(
            Predicate::name(&FnPredicate::from(|_err: &usize| true),),
            "Fn"
        );
    }
}
