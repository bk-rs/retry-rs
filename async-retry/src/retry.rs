use core::{
    fmt,
    future::Future,
    marker::PhantomData,
    ops::ControlFlow,
    pin::Pin,
    task::{Context, Poll},
};

use async_sleep::{sleep, Sleepble};
use futures_util::{future::FusedFuture, pin_mut, FutureExt as _};
use pin_project_lite::pin_project;
use retry_policy::RetryPolicy;

use crate::error::Error;

//
pin_project! {
    #[derive(Debug)]
    pub struct Retry<SLEEP, POL, F, Fut, T, E> {
        policy: POL,
        future_repeater: F,
        //
        state: State,
        attempts: usize,
        errors: Option<Vec<E>>,
        //
        phantom: PhantomData<(SLEEP, Fut, T, E)>,
    }
}

impl<SLEEP, POL, F, Fut, T, E> Retry<SLEEP, POL, F, Fut, T, E> {
    fn new(policy: POL, future_repeater: F) -> Self {
        Self {
            policy,
            future_repeater,
            //
            state: State::Pending,
            attempts: 0,
            errors: Some(vec![]),
            //
            phantom: PhantomData,
        }
    }
}

//
enum State {
    Pending,
    Sleep(Pin<Box<dyn Future<Output = ()>>>),
    Done,
}
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Pending => write!(f, "Pending"),
            State::Sleep(_) => write!(f, "Sleep"),
            State::Done => write!(f, "Done"),
        }
    }
}

//
pub fn retry<SLEEP, POL, F, Fut, T, E>(
    policy: POL,
    future_repeater: F,
) -> Retry<SLEEP, POL, F, Fut, T, E>
where
    SLEEP: Sleepble + 'static,
    POL: RetryPolicy<E>,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    Retry::new(policy, future_repeater)
}

//
impl<SLEEP, POL, F, Fut, T, E> FusedFuture for Retry<SLEEP, POL, F, Fut, T, E>
where
    SLEEP: Sleepble + 'static,
    POL: RetryPolicy<E>,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    fn is_terminated(&self) -> bool {
        matches!(self.state, State::Done)
    }
}

//
impl<SLEEP, POL, F, Fut, T, E> Future for Retry<SLEEP, POL, F, Fut, T, E>
where
    SLEEP: Sleepble + 'static,
    POL: RetryPolicy<E>,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    type Output = Result<T, Error<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        loop {
            match this.state {
                State::Pending => {
                    let future = (this.future_repeater)();
                    pin_mut!(future);

                    match future.poll(cx) {
                        Poll::Ready(Ok(x)) => {
                            //
                            *this.state = State::Done;
                            *this.attempts = 0;
                            *this.errors = Some(vec![]);

                            break Poll::Ready(Ok(x));
                        }
                        Poll::Ready(Err(err)) => {
                            //
                            *this.attempts += 1;

                            //
                            let ret = this.policy.next_step(&err, *this.attempts);

                            //
                            if let Some(errors) = this.errors.as_mut() {
                                errors.push(err)
                            }

                            match ret {
                                ControlFlow::Continue(dur) => {
                                    //
                                    *this.state = State::Sleep(Box::pin(sleep::<SLEEP>(dur)));

                                    continue;
                                }
                                ControlFlow::Break(stop_reason) => {
                                    let errors = this.errors.take().expect("unreachable!()");

                                    //
                                    *this.state = State::Done;
                                    *this.attempts = 0;
                                    *this.errors = Some(vec![]);

                                    break Poll::Ready(Err(Error::new(stop_reason, errors)));
                                }
                            }
                        }
                        Poll::Pending => break Poll::Pending,
                    }
                }
                State::Sleep(future) => match future.poll_unpin(cx) {
                    Poll::Ready(_) => {
                        //
                        *this.state = State::Pending;

                        continue;
                    }
                    Poll::Pending => break Poll::Pending,
                },
                State::Done => panic!("cannot poll Retry twice"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };
    use std::time::Instant;

    use async_sleep::impl_tokio::Sleep;
    use once_cell::sync::Lazy;
    use retry_policy::{
        policies::SimplePolicy,
        retry_backoff::backoffs::FnBackoff,
        retry_predicate::predicates::{AlwaysPredicate, FnPredicate},
        StopReason,
    };

    #[tokio::test]
    async fn test_retry_with_max_retries_reached() {
        #[derive(Debug, PartialEq)]
        struct FError(usize);
        async fn f(n: usize) -> Result<(), FError> {
            Err(FError(n))
        }

        //
        let policy = SimplePolicy::new(
            AlwaysPredicate,
            3,
            FnBackoff::from(|_| Duration::from_millis(100)),
        );

        //
        let now = Instant::now();

        match retry::<Sleep, _, _, _, (), _>(policy, || f(0)).await {
            Ok(_) => panic!(""),
            Err(err) => {
                assert_eq!(&err.stop_reason, &StopReason::MaxRetriesReached);
                assert_eq!(err.errors(), &[FError(0), FError(0), FError(0), FError(0)]);
            }
        }

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 305);
    }

    #[tokio::test]
    async fn test_retry_with_predicate_failed() {
        #[derive(Debug, PartialEq)]
        struct FError(usize);
        async fn f(n: usize) -> Result<(), FError> {
            Err(FError(n))
        }

        //
        static N: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

        let policy = SimplePolicy::new(
            FnPredicate::from(|FError(n): &FError| vec![0, 1].contains(n)),
            3,
            FnBackoff::from(|_| Duration::from_millis(100)),
        );

        //
        let now = Instant::now();

        match retry::<Sleep, _, _, _, (), _>(policy, || f(N.fetch_add(1, Ordering::SeqCst))).await {
            Ok(_) => panic!(""),
            Err(err) => {
                assert_eq!(&err.stop_reason, &StopReason::PredicateFailed);
                assert_eq!(err.errors(), &[FError(0), FError(1), FError(2)]);
            }
        }

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 200 && elapsed_dur.as_millis() <= 205);
    }
}
