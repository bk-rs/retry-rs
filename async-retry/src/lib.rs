use core::future::Future;

use async_sleep::{sleep, Sleepble};
use futures_util::FutureExt as _;
use retry_policy::RetryPolicy;

//
pub mod error;

use error::Error;

//

pub async fn retry<POL, Fut, T, E, SLEEP>(policy: POL, future: Fut) -> Result<T, Error<E>>
where
    POL: RetryPolicy<E>,
    Fut: Future<Output = Result<T, E>>,
    T: Clone,
    E: Clone,
    SLEEP: Sleepble,
{
    todo!()
}
