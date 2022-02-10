use core::fmt;

//
pub mod always;
pub mod r#fn;
pub mod never;

pub use always::AlwaysPredicate;
pub use never::NeverPredicate;
pub use r#fn::FnPredicate;

//
pub trait Predicate<E> {
    /// returns true if a retry; false otherwise
    fn require_retry(&self, err: &E) -> bool;

    fn name(&self) -> &str {
        "_"
    }
}

impl<E> fmt::Debug for dyn Predicate<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Predicate")
            .field(&Predicate::name(self))
            .finish()
    }
}
impl<E> fmt::Debug for dyn Predicate<E> + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Predicate")
            .field(&Predicate::name(self))
            .finish()
    }
}
