//! This module contains all the relevant enums that are mapped to possible
//! responses when attempting to connect to `CCash` and error responses from
//! `CCash`.

use reqwest::Response;
use thiserror::Error;

/// Enum for all the possible responses from the `CCash` endpoints.
#[derive(Debug)]
pub enum CCashResponse {
    /// A successful response from the `CCash` instance.
    Success {
        /// The status code of the `CCash` response (200-299).
        code: u16,
        /// The message of the `CCash` response, if any.
        message: String,
    },
    /// An unsuccessful response from the `CCash` instance.
    Error {
        /// The status code of the `CCash` response (400-499).
        code: u16,
        /// The status code of the `CCash` response.
        message: String,
    },
}

impl std::fmt::Display for CCashResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl CCashResponse {
    pub(crate) async fn from_response(response: Response) -> CCashResponse {
        if response.status().is_success() {
            Self::Success {
                code: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            }
        } else {
            Self::Error {
                code: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            }
        }
    }

    pub(crate) fn convert_message<'de, T: serde::Deserialize<'de> + Default>(
        &'de self,
    ) -> Result<T, ()> {
        if let Self::Success { message, .. } = self {
            serde_json::from_str::<Option<T>>(message)
                .unwrap_or_default()
                .map_or_else(|| Ok(Default::default()), Ok)
        } else {
            Err(())
        }
    }
}

/// Enum for all errors that could occur when receiving a response from a
/// `CCash` instance.
#[derive(Error, Debug)]
pub enum CCashError {
    /// A reqwest error.
    #[error("reqwest encontered an error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// An error when unable to connect to the `CCash` instance.
    #[error(
        "Client connection has not been established. Make sure establish_connection has \
         been called and has returned a positive response."
    )]
    ConnectionNotAvailable,
    /// An error when the API was unable to parse the properties returned by the
    /// `CCash` instance. Could be caused by an incompatible `CCash` instance
    /// version.
    #[error(
        "Bank server returned value that could not be parsed correctly. Contact the \
         server admin."
    )]
    CouldNotParsePropertiesResponse,
    /// An error returned by the `CCash` instance itself.
    #[error("The `CCash` server responded with {0}")]
    ErrorResponse(CCashResponse),
    /// An returned if `ccash-rs` runs into an internal problem.
    #[error("ccash-rs ran into a problem: {0}")]
    Error(String),
}

impl From<CCashResponse> for CCashError {
    fn from(r: CCashResponse) -> Self { CCashError::ErrorResponse(r) }
}
