use serde_json;
use hyper;

/// Kinds of errors that could happen at runtime.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while parsing the received JSON
    ParsingError(serde_json::error::Error),

    /// An error occurred during a request to the APIs
    HTTPError(hyper::error::Error),

    /// An error returned by the APIs
    LastFMError(LastFMErrorResponse)
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
    RateLimitExceeded(LastFMError)
}

/// A generic LastFM response when the request can't be accomplished.
#[derive(Deserialize, Debug)]
pub struct LastFMError {
    pub error:   i32,
    pub message: String,
    pub links:   Vec<String>
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
            _  => LastFMErrorResponse::GenericError(lastm_error)
        }
    }
}
