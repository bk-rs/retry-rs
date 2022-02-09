use core::fmt;

use http::StatusCode;

use super::{default_backoff, Policy};

/// [Object: http.default_retry](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry)
pub fn default_retry() -> Policy<Error> {
    Policy::new(default_retry_predicate, 5, default_backoff())
}

/// [Object: http.default_retry_non_idempotent](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_non_idempotent)
pub fn default_retry_non_idempotent() -> Policy<Error> {
    Policy::new(default_retry_predicate_non_idempotent, 5, default_backoff())
}

pub const RETRIES_ON_STATUS_CODES: &[StatusCode] = &[
    StatusCode::TOO_MANY_REQUESTS,
    StatusCode::BAD_GATEWAY,
    StatusCode::SERVICE_UNAVAILABLE,
    StatusCode::GATEWAY_TIMEOUT,
];

/// [Function: http.default_retry_predicate](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_predicate)
pub fn default_retry_predicate(err: &Error) -> bool {
    match err {
        Error::StatusCode(status_code) => RETRIES_ON_STATUS_CODES.contains(status_code),
        Error::ConnectionError => true,
        Error::TimeoutError => true,
    }
}

pub const RETRIES_ON_STATUS_CODES_NON_IDEMPOTENT: &[StatusCode] = &[
    StatusCode::TOO_MANY_REQUESTS,
    StatusCode::SERVICE_UNAVAILABLE,
];

/// [Function: http.default_retry_predicate_non_idempotent](https://cloud.google.com/workflows/docs/reference/stdlib/http/default_retry_predicate_non_idempotent)
pub fn default_retry_predicate_non_idempotent(err: &Error) -> bool {
    match err {
        Error::StatusCode(status_code) => {
            RETRIES_ON_STATUS_CODES_NON_IDEMPOTENT.contains(status_code)
        }
        Error::ConnectionError => true,
        Error::TimeoutError => true,
    }
}

//
#[derive(Debug)]
pub enum Error {
    StatusCode(StatusCode),
    ConnectionError,
    TimeoutError,
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
        for status_code in &[429, 502, 503, 504] {
            assert!((policy.predicate)(&Error::StatusCode(
                StatusCode::try_from(*status_code).unwrap()
            )));
        }
        for status_code in &[200, 401] {
            assert!(!(policy.predicate)(&Error::StatusCode(
                StatusCode::try_from(*status_code).unwrap()
            )));
        }
        assert!((policy.predicate)(&Error::ConnectionError));
        assert!((policy.predicate)(&Error::TimeoutError));
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.backoff, default_backoff());
    }

    #[test]
    fn test_default_retry_non_idempotent() {
        let policy = default_retry_non_idempotent();
        for status_code in &[429, 503] {
            assert!((policy.predicate)(&Error::StatusCode(
                StatusCode::try_from(*status_code).unwrap()
            )));
        }
        for status_code in &[200, 401, 502, 504] {
            assert!(!(policy.predicate)(&Error::StatusCode(
                StatusCode::try_from(*status_code).unwrap()
            )));
        }
        assert!((policy.predicate)(&Error::ConnectionError));
        assert!((policy.predicate)(&Error::TimeoutError));
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.backoff, default_backoff());
    }
}
