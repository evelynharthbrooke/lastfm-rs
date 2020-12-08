use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Kinds of errors that could happen at runtime.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while parsing the received JSON
    ParsingError(serde_json::error::Error),

    /// An error occurred during a request to the APIs
    HTTPError(reqwest::Error),

    /// An error returned by the APIs
    LastFMError(LastFMErrorResponse),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::ParsingError(ref inner) => inner.fmt(f),
            Error::HTTPError(ref inner) => inner.fmt(f),
            Error::LastFMError(ref inner) => inner.fmt(f),
        }
    }
}

/// Representation of all the LastFM APIs errors
#[derive(Debug)]
pub enum LastFMErrorResponse {
    InvalidService(LastFMError),
    InvalidMethod(LastFMError),
    AuthenticationFailed(LastFMError),
    InvalidFormat(LastFMError),
    InvalidParameter(LastFMError),
    InvalidResourceSpecified(LastFMError),
    OperationFailed(LastFMError),
    InvalidSessionKey(LastFMError),
    InvalidAPIKey(LastFMError),
    ServiceOffline(LastFMError),
    InvalidMethodSignatureSupplied(LastFMError),
    GenericError(LastFMError),
    SuspendedAPIKey(LastFMError),
    RateLimitExceeded(LastFMError),
}

impl Display for LastFMErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            LastFMErrorResponse::InvalidService(ref inner) => write!(f, "InvalidService: {}", inner.message),
            LastFMErrorResponse::InvalidMethod(ref inner) => write!(f, "InvalidMethod: {}", inner.message),
            LastFMErrorResponse::AuthenticationFailed(ref inner) => write!(f, "AuthenticationFailed: {}", inner.message),
            LastFMErrorResponse::InvalidFormat(ref inner) => write!(f, "InvalidFormat: {}", inner.message),
            LastFMErrorResponse::InvalidParameter(ref inner) => write!(f, "InvalidParameter: {}", inner.message),
            LastFMErrorResponse::InvalidResourceSpecified(ref inner) => write!(f, "InvalidResourceSpecified: {}", inner.message),
            LastFMErrorResponse::OperationFailed(ref inner) => write!(f, "OperationFailed: {}", inner.message),
            LastFMErrorResponse::InvalidSessionKey(ref inner) => write!(f, "InvalidSessionKey: {}", inner.message),
            LastFMErrorResponse::InvalidAPIKey(ref inner) => write!(f, "InvalidAPIKey: {}", inner.message),
            LastFMErrorResponse::ServiceOffline(ref inner) => write!(f, "ServiceOffline: {}", inner.message),
            LastFMErrorResponse::InvalidMethodSignatureSupplied(ref inner) => write!(f, "InvalidMethodSignatureSupplied: {}", inner.message),
            LastFMErrorResponse::GenericError(ref inner) => write!(f, "GenericError: {}", inner.message),
            LastFMErrorResponse::SuspendedAPIKey(ref inner) => write!(f, "SuspendedAPIKey: {}", inner.message),
            LastFMErrorResponse::RateLimitExceeded(ref inner) => write!(f, "RateLimitExceeded: {}", inner.message),
        }
    }
}

/// A generic LastFM response when the request can't be accomplished.
#[derive(Deserialize, Debug)]
pub struct LastFMError {
    pub error: i32,
    pub message: String,
    pub links: Option<Vec<String>>,
}

impl From<LastFMError> for LastFMErrorResponse {
    fn from(lastm_error: LastFMError) -> LastFMErrorResponse {
        match lastm_error.error {
            2 => LastFMErrorResponse::InvalidService(lastm_error),
            3 => LastFMErrorResponse::InvalidMethod(lastm_error),
            4 => LastFMErrorResponse::AuthenticationFailed(lastm_error),
            5 => LastFMErrorResponse::InvalidFormat(lastm_error),
            6 => LastFMErrorResponse::InvalidParameter(lastm_error),
            7 => LastFMErrorResponse::InvalidResourceSpecified(lastm_error),
            8 => LastFMErrorResponse::OperationFailed(lastm_error),
            9 => LastFMErrorResponse::InvalidSessionKey(lastm_error),
            10 => LastFMErrorResponse::InvalidAPIKey(lastm_error),
            11 => LastFMErrorResponse::ServiceOffline(lastm_error),
            13 => LastFMErrorResponse::InvalidMethodSignatureSupplied(lastm_error),
            16 => LastFMErrorResponse::GenericError(lastm_error),
            26 => LastFMErrorResponse::SuspendedAPIKey(lastm_error),
            29 => LastFMErrorResponse::RateLimitExceeded(lastm_error),
            _ => LastFMErrorResponse::GenericError(lastm_error),
        }
    }
}
