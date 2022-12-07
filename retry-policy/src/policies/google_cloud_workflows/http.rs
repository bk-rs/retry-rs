use retry_backoff::backoffs::google_cloud_workflows::Backoff;
use retry_predicate::predicates::FnPredicate;

use super::Policy;

/// [Object: http.default_retry](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry)
pub fn default_retry() -> Policy<Error> {
    Policy::new(
        FnPredicate::from(default_retry_predicate),
        5,
        Backoff::default(),
    )
}

/// [Object: http.default_retry_non_idempotent](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_non_idempotent)
pub fn default_retry_non_idempotent() -> Policy<Error> {
    Policy::new(
        FnPredicate::from(default_retry_predicate_non_idempotent),
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
    Other,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[cfg(feature = "std")]
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
            assert!(policy.predicate.test(err));
        }
        #[allow(clippy::single_element_loop)]
        for err in &[Error::Other] {
            assert!(!policy.predicate.test(err));
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
            assert!(policy.predicate.test(err));
        }
        for err in &[
            Error::BadGateway,
            Error::GatewayTimeout,
            Error::ConnectionError,
            Error::TimeoutError,
            Error::Other,
        ] {
            assert!(!policy.predicate.test(err));
        }
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.backoff, Backoff::default());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_error_display() {
        assert_eq!(alloc::format!("{}", Error::BadGateway), "BadGateway")
    }
}
