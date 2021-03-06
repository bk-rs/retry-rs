#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

//
pub mod retry_predicate;

pub use self::retry_predicate::RetryPredicate;

//
pub mod predicates;
