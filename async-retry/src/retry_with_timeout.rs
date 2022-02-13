use alloc::boxed::Box;
use core::{convert::Infallible, fmt, future::Future, time::Duration};

use async_sleep::{
    timeout::{timeout, Error as TimeoutError},
    Sleepble,
};
use futures_util::TryFutureExt as _;
use retry_policy::{retry_predicate::RetryPredicate, RetryPolicy};

use crate::retry::Retry;

//
pub fn retry_with_timeout<SLEEP, POL, F, Fut, T, E>(
    policy: POL,
    future_repeater: F,
    every_performance_timeout_dur: Duration,
) -> Retry<SLEEP, POL, T, ErrorWrapper<E>>
where
    SLEEP: Sleepble + 'static,
    POL: RetryPolicy<ErrorWrapper<E>>,
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    Retry::<SLEEP, _, _, _>::new(
        policy,
        Box::new(move || {
            let fut = future_repeater();
            Box::pin(
                timeout::<SLEEP, _>(every_performance_timeout_dur, Box::pin(fut)).map_ok_or_else(
                    |err| Err(ErrorWrapper::Timeout(err)),
                    |ret| match ret {
                        Ok(x) => Ok(x),
                        Err(err) => Err(ErrorWrapper::Inner(err)),
                    },
                ),
            )
        }),
    )
}

//
pub fn retry_with_timeout_for_unresult<SLEEP, POL, F, Fut, T>(
    policy: POL,
    future_repeater: F,
    every_performance_timeout_dur: Duration,
) -> Retry<SLEEP, POL, T, ErrorWrapper<Infallible>>
where
    SLEEP: Sleepble + 'static,
    POL: RetryPolicy<ErrorWrapper<Infallible>>,
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = T> + 'static,
{
    Retry::<SLEEP, _, _, _>::new(
        policy,
        Box::new(move || {
            let fut = future_repeater();
            Box::pin(
                timeout::<SLEEP, _>(every_performance_timeout_dur, Box::pin(fut))
                    .map_ok_or_else(|err| Err(ErrorWrapper::Timeout(err)), |x| Ok(x)),
            )
        }),
    )
}

//
//
//
pub enum ErrorWrapper<T> {
    Inner(T),
    Timeout(TimeoutError),
}

impl<T> fmt::Debug for ErrorWrapper<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorWrapper::Inner(err) => f.debug_tuple("ErrorWrapper::Inner").field(err).finish(),
            ErrorWrapper::Timeout(err) => {
                f.debug_tuple("ErrorWrapper::Timeout").field(err).finish()
            }
        }
    }
}

impl<T> fmt::Display for ErrorWrapper<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl<T> std::error::Error for ErrorWrapper<T> where T: fmt::Debug {}

impl<T> ErrorWrapper<T> {
    pub fn is_inner(&self) -> bool {
        matches!(self, Self::Inner(_))
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }

    pub fn into_inner(self) -> Option<T> {
        match self {
            Self::Inner(x) => Some(x),
            Self::Timeout(_) => None,
        }
    }
}

//
//
//
pub struct PredicateWrapper<T> {
    inner: T,
}

impl<T> fmt::Debug for PredicateWrapper<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PredicateWrapper")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> PredicateWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<E, P> RetryPredicate<ErrorWrapper<E>> for PredicateWrapper<P>
where
    P: RetryPredicate<E>,
{
    fn test(&self, params: &ErrorWrapper<E>) -> bool {
        match params {
            ErrorWrapper::Inner(inner_params) => self.inner.test(inner_params),
            ErrorWrapper::Timeout(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;
    use core::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    use async_sleep::impl_tokio::Sleep;
    use once_cell::sync::Lazy;
    use retry_policy::{
        policies::SimplePolicy,
        retry_backoff::backoffs::FnBackoff,
        retry_predicate::predicates::{AlwaysPredicate, FnPredicate},
        StopReason,
    };

    #[tokio::test]
    async fn test_retry_with_timeout() {
        #[derive(Debug, PartialEq)]
        struct FError(usize);
        async fn f(n: usize) -> Result<(), FError> {
            #[allow(clippy::single_match)]
            match n {
                1 => tokio::time::sleep(tokio::time::Duration::from_millis(80)).await,
                _ => {}
            }
            Err(FError(n))
        }

        //
        static N: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

        let policy = SimplePolicy::new(
            PredicateWrapper::new(FnPredicate::from(|FError(n): &FError| {
                vec![0, 1].contains(n)
            })),
            3,
            FnBackoff::from(|_| Duration::from_millis(100)),
        );

        //
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        match retry_with_timeout::<Sleep, _, _, _, _, _>(
            policy,
            || f(N.fetch_add(1, Ordering::SeqCst)),
            Duration::from_millis(50),
        )
        .await
        {
            Ok(_) => panic!(""),
            Err(err) => {
                assert_eq!(&err.stop_reason, &StopReason::PredicateFailed);
                for (i, err) in err.errors().iter().enumerate() {
                    #[cfg(feature = "std")]
                    println!("{} {:?}", i, err);
                    match i {
                        0 => match err {
                            ErrorWrapper::Inner(FError(n)) => {
                                assert_eq!(*n, 0)
                            }
                            err => panic!("{} {:?}", i, err),
                        },
                        1 => match err {
                            ErrorWrapper::Timeout(TimeoutError::Timeout(dur)) => {
                                assert_eq!(*dur, Duration::from_millis(50));
                            }
                            err => panic!("{} {:?}", i, err),
                        },
                        2 => match err {
                            ErrorWrapper::Inner(FError(n)) => {
                                assert_eq!(*n, 2)
                            }
                            err => panic!("{} {:?}", i, err),
                        },
                        n => panic!("{} {:?}", n, err),
                    }
                }
            }
        }

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 250 && elapsed_dur.as_millis() <= 260);
        }
    }

    #[tokio::test]
    async fn test_retry_with_timeout_for_unresult() {
        async fn f(n: usize) {
            #[allow(clippy::single_match)]
            match n {
                0 => tokio::time::sleep(tokio::time::Duration::from_millis(80)).await,
                _ => {}
            }
        }

        //
        static N: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

        let policy = SimplePolicy::new(
            PredicateWrapper::new(AlwaysPredicate),
            3,
            FnBackoff::from(|_| Duration::from_millis(100)),
        );

        //
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        match retry_with_timeout_for_unresult::<Sleep, _, _, _, ()>(
            policy,
            || f(N.fetch_add(1, Ordering::SeqCst)),
            Duration::from_millis(50),
        )
        .await
        {
            Ok(_) => {}
            Err(err) => {
                panic!("{:?}", err)
            }
        }

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 150 && elapsed_dur.as_millis() <= 155);
        }
    }

    #[tokio::test]
    async fn test_retry_with_timeout_for_unresult_with_max_retries_reached() {
        async fn f(_n: usize) {
            tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
        }

        //
        static N: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

        let policy = SimplePolicy::new(
            PredicateWrapper::new(AlwaysPredicate),
            3,
            FnBackoff::from(|_| Duration::from_millis(100)),
        );

        //
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        match retry_with_timeout_for_unresult::<Sleep, _, _, _, ()>(
            policy,
            || f(N.fetch_add(1, Ordering::SeqCst)),
            Duration::from_millis(50),
        )
        .await
        {
            Ok(_) => panic!(""),
            Err(err) => {
                assert_eq!(&err.stop_reason, &StopReason::MaxRetriesReached);
                for (i, err) in err.errors().iter().enumerate() {
                    #[cfg(feature = "std")]
                    println!("{} {:?}", i, err);
                    match i {
                        0 | 1 | 2 | 3 => match err {
                            ErrorWrapper::Timeout(TimeoutError::Timeout(dur)) => {
                                assert_eq!(*dur, Duration::from_millis(50));
                            }
                            err => panic!("{} {:?}", i, err),
                        },

                        n => panic!("{} {:?}", n, err),
                    }
                }
            }
        }

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 500 && elapsed_dur.as_millis() <= 515);
        }
    }
}
