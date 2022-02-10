use core::fmt;

use retry_backoff::backoffs::google_cloud_workflows::Backoff;

use super::Policy;

/// [Object: http.default_retry](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry)
pub fn default_retry() -> Policy<Error> {
    Policy::new(default_retry_predicate, 5, Backoff::default())
}

/// [Object: http.default_retry_non_idempotent](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_non_idempotent)
pub fn default_retry_non_idempotent() -> Policy<Error> {
    Policy::new(
        default_retry_predicate_non_idempotent,
        5,
        Backoff::default(),
    )
}

/// [Function: http.default_retry_predicate](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_predicate)
pub fn default_retry_predicate(err: &Error) -> bool {
    matches!(
        err,
        Error::TooManyRequests {
            retry_after_delay_seconds: _
        } | &Error::BadGateway
            | Error::ServiceUnavailable {
                retry_after_delay_seconds: _
            }
            | &Error::GatewayTimeout
            | Error::ConnectionError
            | Error::TimeoutError
    )
}

/// [Function: http.default_retry_predicate_non_idempotent](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_predicate_non_idempotent)
pub fn default_retry_predicate_non_idempotent(err: &Error) -> bool {
    matches!(
        err,
        Error::TooManyRequests {
            retry_after_delay_seconds: _
        } | Error::ServiceUnavailable {
            retry_after_delay_seconds: _
        }
    )
}

//
#[derive(Debug)]
pub enum Error {
    /// 429
    TooManyRequests {
        /// [Ref](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After)
        retry_after_delay_seconds: Option<usize>,
    },
    /// 502
    BadGateway,
    /// 503
    ServiceUnavailable {
        /// [Ref](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After)
        retry_after_delay_seconds: Option<usize>,
    },
    /// 504
    GatewayTimeout,
    ConnectionError,
    TimeoutError,
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_retry() {
        let policy = default_retry();
        for err in &[
            Error::TooManyRequests {
                retry_after_delay_seconds: None,
            },
            Error::BadGateway,
            Error::ServiceUnavailable {
                retry_after_delay_seconds: None,
            },
            Error::GatewayTimeout,
            Error::ConnectionError,
            Error::TimeoutError,
        ] {
            assert!((policy.predicate)(err));
        }
        for err in &[Error::Other(Box::new(std::io::Error::from(
            std::io::ErrorKind::TimedOut,
        )))] {
            assert!(!(policy.predicate)(err));
        }
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.backoff, Backoff::default());
    }

    #[test]
    fn test_default_retry_non_idempotent() {
        let policy = default_retry_non_idempotent();
        for err in &[
            Error::TooManyRequests {
                retry_after_delay_seconds: None,
            },
            Error::ServiceUnavailable {
                retry_after_delay_seconds: None,
            },
        ] {
            assert!((policy.predicate)(err));
        }
        for err in &[
            Error::BadGateway,
            Error::GatewayTimeout,
            Error::ConnectionError,
            Error::TimeoutError,
            Error::Other(Box::new(std::io::Error::from(std::io::ErrorKind::TimedOut))),
        ] {
            assert!(!(policy.predicate)(err));
        }
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.backoff, Backoff::default());
    }
}
