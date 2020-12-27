//! Last.fm Error Handling
//!
//! This module handles any and all errors related to Last.fm, such as when
//! authentication fails, the provided API key is invalid, and various other
//! errors. Please check the [LastFMErrorResponse] struct for more information
//! about the various types of errors the Last.fm API transmits when the API encounters
//! an error.
//!
//! [LastFMErrorResponse]: crate::error::LastFMErrorResponse

use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Kinds of errors that could happen at runtime.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while parsing the received JSON
    ParsingError(serde_json::error::Error),
    /// An error occurred while a request was being made to the API.
    HTTPError(reqwest::Error),
    /// An error returned by the Last.fm API.
    LastFMError(LastFMErrorResponse),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> { Some(self) }
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

/// Representation of all the errors exposed by the Last.fm API.
#[derive(Debug)]
pub enum LastFMErrorResponse {
    /// Invalid Service - This service does not exist.
    InvalidService(LastFMError),
    /// Invalid Method - No method exists by the name provided.
    InvalidMethod(LastFMError),
    /// Authentication Failed - Failed to authenticate with the Last.fm API.
    AuthenticationFailed(LastFMError),
    /// Invalid Format - Service does not exist in the format given.
    InvalidFormat(LastFMError),
    /// Invalid Parameters - A required parameter is missing from the request,
    /// or one or more parameters are invalid.
    InvalidParameters(LastFMError),
    /// Invalid Resource Specified - An invalid resource was specified.
    InvalidResourceSpecified(LastFMError),
    /// Operation Failed - Something else went wrong.
    OperationFailed(LastFMError),
    /// Invalid Session Key - Please re-authenticate with the Last.fm API.
    InvalidSessionKey(LastFMError),
    /// Invalid API Key - An invalid API key was provided.
    InvalidAPIKey(LastFMError),
    /// Service Offline - The given service is temporarily offline. Try again later.
    ServiceOffline(LastFMError),
    /// Invalid Method Signature Supplied - An invalid signature for the given methoid was supplied.
    InvalidMethodSignatureSupplied(LastFMError),
    /// Generic Error - An unknown error has occurred.
    GenericError(LastFMError),
    /// Suspended API Key - The given API key has been suspended.
    SuspendedAPIKey(LastFMError),
    /// Rate Limit Exceeded - The rate limit for this API key has been exceeded.
    RateLimitExceeded(LastFMError),
}

impl Display for LastFMErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            LastFMErrorResponse::InvalidService(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidMethod(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::AuthenticationFailed(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidFormat(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidParameters(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidResourceSpecified(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::OperationFailed(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidSessionKey(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidAPIKey(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::ServiceOffline(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::InvalidMethodSignatureSupplied(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::GenericError(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::SuspendedAPIKey(ref inner) => write!(f, "{}", inner.message),
            LastFMErrorResponse::RateLimitExceeded(ref inner) => write!(f, "{}", inner.message),
        }
    }
}

/// A generic Last.fm response when the request can't be accomplished.
#[derive(Deserialize, Debug)]
pub struct LastFMError {
    /// The error code associated with the error.
    pub error: i32,
    /// The message associated with the error.
    pub message: String,
    /// Any links associated with the error, if available.
    pub links: Option<Vec<String>>,
}

impl From<LastFMError> for LastFMErrorResponse {
    fn from(lastm_error: LastFMError) -> LastFMErrorResponse {
        match lastm_error.error {
            2 => LastFMErrorResponse::InvalidService(lastm_error),
            3 => LastFMErrorResponse::InvalidMethod(lastm_error),
            4 => LastFMErrorResponse::AuthenticationFailed(lastm_error),
            5 => LastFMErrorResponse::InvalidFormat(lastm_error),
            6 => LastFMErrorResponse::InvalidParameters(lastm_error),
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
