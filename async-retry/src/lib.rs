#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
pub mod error;
pub mod retry;
pub mod retry_with_timeout;

pub use error::Error;
pub use retry::{Retry, retry};
pub use retry_with_timeout::{retry_with_timeout, retry_with_timeout_for_non_logic_error};
