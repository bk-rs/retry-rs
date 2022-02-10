#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

//
mod retry_policy;

pub use self::retry_policy::RetryPolicy;

//
pub mod policies;
