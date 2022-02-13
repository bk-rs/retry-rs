//
#[cfg(feature = "alloc")]
mod r#fn;

#[cfg(feature = "alloc")]
pub use r#fn::Backoff as FnBackoff;

//
pub mod google_cloud_workflows;

pub use google_cloud_workflows::Backoff as GoogleCloudWorkflowsBackoff;

//
#[cfg(feature = "impl_exponential_backoff")]
pub mod impl_exponential_backoff;

#[cfg(feature = "impl_backoff_rs")]
pub mod impl_backoff_rs;
