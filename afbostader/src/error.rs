use reqwest::StatusCode;
use serde::Deserialize;

mod status_serde {
    use reqwest::StatusCode;
    use serde::{de, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let status = u16::deserialize(deserializer)?;
        StatusCode::from_u16(status).map_err(|_| de::Error::custom("invalid status code"))
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorResponse {
    #[serde(with = "status_serde")]
    status: StatusCode,
    // error: String,
    message: Option<String>,
    cause_en: Option<String>,
}

impl ErrorResponse {
    fn into_message(self) -> Option<String> {
        self.message.or(self.cause_en)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] reqwest_middleware::Error),
    #[error("bad credentials")]
    BadCredentials,
    #[error("unauthenticated")]
    Unauthenticated,
    #[error("unknown api error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value.into())
    }
}

impl From<ErrorResponse> for Error {
    fn from(value: ErrorResponse) -> Self {
        let status = value.status;

        match status {
            StatusCode::UNAUTHORIZED => Self::BadCredentials,
            _ => Self::Unknown(value.into_message().unwrap_or_else(|| status.to_string())),
        }
    }
}
