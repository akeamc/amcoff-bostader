use error::{Error, ErrorResponse};
use reqwest::IntoUrl;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ")"
);

mod error;
mod model;

pub use model::*;
use serde_json::Value;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct Credentials {
    email: String,
    password: SecretString,
}

impl Credentials {
    pub fn new(email: impl Into<String>, password: impl Into<SecretString>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: ClientWithMiddleware,
    credentials: Option<Credentials>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(5);
        let client = ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            inner: client,
            credentials: None,
        }
    }

    pub fn with_credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    fn get(&self, url: impl IntoUrl) -> RequestBuilder {
        let builder = self.inner.get(url);

        if let Some(ref credentials) = self.credentials {
            builder.basic_auth(
                credentials.email.clone(),
                Some(credentials.password.expose_secret()),
            )
        } else {
            builder
        }
    }

    pub async fn list_vacant(&self) -> Result<Vec<Property>, Error> {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum Response {
            Product { product: Vec<Product> },
            Error(ErrorResponse),
        }

        match self
            .get("https://diremoapi.afbostader.se/redimo/rest/vacantproducts?lang=sv_SE&type=1")
            .send()
            .await?
            .json::<Response>()
            .await?
        {
            Response::Product { product } => Ok(product.into_iter().map(Into::into).collect()),
            Response::Error(e) => Err(e.into()),
        }
    }

    pub async fn vacant_detail(&self, id: PropertyId) -> Result<PropertyDetail, Error> {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        #[allow(clippy::large_enum_variant)]
        enum Response {
            Product(ProductDetail),
            Error(ErrorResponse),
        }

        match self
            .get(format!(
                "https://diremoapi.afbostader.se/redimo/rest/vacantproducts/{id}?lang=sv_SE"
            ))
            .send()
            .await?
            .json::<Response>()
            .await?
        {
            Response::Product(product) => Ok(product.into()),
            Response::Error(e) => Err(e.into()),
        }
    }

    pub async fn user_info(&self) -> Result<User, Error> {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        #[allow(clippy::large_enum_variant)]
        enum Response {
            UserInfo(UserInfo),
            Error(ErrorResponse),
            Strange(Value),
        }

        if self.credentials.is_none() {
            warn!("requesting user info without credentials");
        }

        match self
            .get("https://diremoapi.afbostader.se/redimo/rest/registerForHousing/getUserInfo")
            .send()
            .await?
            .json::<Response>()
            .await?
        {
            Response::UserInfo(info) => Ok(info.into()),
            Response::Error(e) => Err(e.into()),
            Response::Strange(v) => {
                if let Some(obj) = v.as_object() {
                    if !obj.is_empty() && obj.values().all(Value::is_null) {
                        return Err(Error::Unauthenticated);
                    }
                }

                Err(Error::Unknown(format!(
                    "unexpected json from server: {v:#?}"
                )))
            }
        }
    }
}
