//
#[cfg(feature = "alloc")]
mod r#fn;
#[cfg(feature = "alloc")]
mod simple;

#[cfg(feature = "alloc")]
pub use r#fn::Policy as FnPolicy;
#[cfg(feature = "alloc")]
pub use simple::Policy as SimplePolicy;

//
#[cfg(feature = "alloc")]
pub mod google_cloud_workflows;

#[cfg(feature = "alloc")]
pub use google_cloud_workflows::Policy as GoogleCloudWorkflowsPolicy;
