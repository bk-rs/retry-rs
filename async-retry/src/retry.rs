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

use crate::error::Error;

//
pin_project! {
    #[derive(Debug)]
    pub struct Retry<POL, PParams, F, Fut, T, E, SLEEP> {
        policy: POL,
        future_repeater: F,
        //
        state: State,
        attempts: usize,
        errors: Option<Vec<E>>,
        //
        phantom: PhantomData<(PParams, Fut, T, E, SLEEP)>,
    }
}

impl<POL, PParams, F, Fut, T, E, SLEEP> Retry<POL, PParams, F, Fut, T, E, SLEEP> {
    fn new(policy: POL, future_repeater: F) -> Self {
        Self {
            policy,
            future_repeater,
            state: State::default(),
            attempts: 0,
            errors: Some(vec![]),
            phantom: PhantomData,
        }
    }
}

enum State {
    Pending,
    Sleep(Pin<Box<dyn Future<Output = ()>>>),
    Done,
}
impl Default for State {
    fn default() -> Self {
        Self::Pending
    }
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
    Retry::new(policy, future_repeater)
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

        loop {
            match this.state {
                State::Pending => {
                    let mut future = (this.future_repeater)();

                    match future.poll_unpin(cx) {
                        Poll::Ready(Ok(x)) => {
                            //
                            *this.state = State::Done;
                            *this.attempts = 0;
                            *this.errors = Some(vec![]);

                            break Poll::Ready(Ok(x));
                        }
                        Poll::Ready(Err(err)) => {
                            let params = PParams::from(&err);

                            //
                            *this.attempts += 1;
                            if let Some(errors) = this.errors.as_mut() {
                                errors.push(err)
                            }

                            match this.policy.next_step(&params, *this.attempts) {
                                ControlFlow::Continue(dur) => {
                                    //
                                    *this.state = State::Sleep(Box::pin(sleep::<SLEEP>(dur)));

                                    break Poll::Pending;
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
                State::Done => panic!("cannot poll Select twice"),
            }
        }
    }
}
