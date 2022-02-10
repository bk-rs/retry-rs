#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

//
mod retry_backoff;

pub use self::retry_backoff::RetryBackoff;

//
pub mod backoffs;
