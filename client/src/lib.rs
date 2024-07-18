use error::{Error, ErrorResponse};
use reqwest::{IntoUrl, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use secrecy::{ExposeSecret, SecretString};
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
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

    pub async fn list_vacancies(&self) -> Result<Vec<Property>, Error> {
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

    pub async fn vacancy_detail(&self, id: PropertyId) -> Result<PropertyDetail, Error> {
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

    pub async fn area_detail(&self, area_name: &str) -> Result<AreaDetail, Error> {
        let base = Url::parse("https://www.afbostader.se").unwrap();

        let html = self
            .inner
            .get(
                base.join("/lediga-bostader/bostadsomraden/")
                    .unwrap()
                    .join(&slug::slugify(area_name))
                    .unwrap(),
            )
            .send()
            .await?
            .text()
            .await?;
        let doc = Document::from(html.as_str());

        let pictures = doc
            .find(
                Class("slideshow")
                    .descendant(Class("slides"))
                    .descendant(Name("img")),
            )
            .filter_map(|node| {
                let alt = node
                    .attr("alt")
                    .map(|s| s.trim_matches('\"'))
                    .filter(|s| !s.is_empty())
                    .map(ToOwned::to_owned);

                Some(Picture {
                    alt,
                    url: base.join(node.attr("src")?).ok()?,
                })
            })
            .collect();

        Ok(AreaDetail { pictures })
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
