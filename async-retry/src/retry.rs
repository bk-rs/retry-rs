use core::{
    fmt,
    future::Future,
    marker::PhantomData,
    ops::ControlFlow,
    pin::Pin,
    task::{Context, Poll},
};

use async_sleep::{sleep, Sleepble};
use futures_util::{future::FusedFuture, FutureExt as _};
use pin_project_lite::pin_project;
use retry_policy::RetryPolicy;

use crate::error::{Error, ErrorKind};

//
pin_project! {
    #[derive(Debug)]
    pub struct Retry<POL, PParams, F, Fut, T, E, SLEEP> {
        policy: POL,
        future_repeater: F,
        state: State,
        attempts: usize,
        phantom: PhantomData<(PParams, Fut, T, E, SLEEP)>,
    }
}

enum State {
    Pending,
    Sleep(Pin<Box<dyn Future<Output = ()>>>),
    #[allow(dead_code)]
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

pub fn retry<POL, PParams, F, Fut, T, E, SLEEP>(
    policy: POL,
    future_repeater: F,
) -> Retry<POL, PParams, F, Fut, T, E, SLEEP>
where
    POL: RetryPolicy<PParams>,
    PParams: for<'a> From<&'a E>,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>> + Unpin,
    SLEEP: Sleepble,
{
    Retry {
        policy,
        future_repeater,
        state: State::Pending,
        attempts: 0,
        phantom: PhantomData,
    }
}

impl<POL, PParams, F, Fut, T, E, SLEEP> FusedFuture for Retry<POL, PParams, F, Fut, T, E, SLEEP>
where
    POL: RetryPolicy<PParams>,
    PParams: for<'a> From<&'a E>,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>> + Unpin,
    SLEEP: Sleepble + 'static,
{
    fn is_terminated(&self) -> bool {
        matches!(self.state, State::Done)
    }
}

impl<POL, PParams, F, Fut, T, E, SLEEP> Future for Retry<POL, PParams, F, Fut, T, E, SLEEP>
where
    POL: RetryPolicy<PParams>,
    PParams: for<'a> From<&'a E>,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>> + Unpin,
    SLEEP: Sleepble + 'static,
{
    type Output = Result<T, Error<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.state {
            State::Pending => {
                let mut future = (this.future_repeater)();

                match future.poll_unpin(cx) {
                    Poll::Ready(Ok(x)) => Poll::Ready(Ok(x)),
                    Poll::Ready(Err(err)) => {
                        let params = PParams::from(&err);

                        match this.policy.next_step(&params, *this.attempts) {
                            ControlFlow::Continue(dur) => {
                                *this.attempts += 1;

                                *this.state = State::Sleep(Box::pin(sleep::<SLEEP>(dur)));

                                Poll::Pending
                            }
                            ControlFlow::Break(stop_reason) => Poll::Ready(Err(Error::new(
                                ErrorKind::RetryPolicyStopReason(stop_reason),
                                err,
                            ))),
                        }
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            State::Sleep(future) => match future.poll_unpin(cx) {
                Poll::Ready(_) => {
                    *this.state = State::Pending;
                    Poll::Pending
                }
                Poll::Pending => Poll::Pending,
            },
            State::Done => panic!("cannot poll Select twice"),
        }
    }
}
