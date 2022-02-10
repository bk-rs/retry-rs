//
#[cfg(feature = "alloc")]
mod r#fn;

#[cfg(feature = "alloc")]
pub use r#fn::Predicate as FnPredicate;

//
mod always;
mod never;

pub use always::Predicate as AlwaysPredicate;
pub use never::Predicate as NeverPredicate;
