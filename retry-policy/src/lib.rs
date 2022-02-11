#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

//
pub use retry_backoff;
pub use retry_predicate;

//
mod retry_policy;

pub use self::retry_policy::{RetryPolicy, StopReason};

//
pub mod policies;
