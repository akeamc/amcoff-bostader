use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use time::Date;

pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

mod as_str {
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        value.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|_| de::Error::custom("from_str failed"))
    }
}

time::serde::format_description!(yyyy_mm_dd, Date, "[year]-[month]-[day]");

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Product {
//     pub product_id: String,
//     #[serde(rename = "type")]
//     pub typ: String,
//     pub status: Option<()>,
//     pub description: String,
//     pub short_description: String,
//     pub area: String,
//     #[serde(rename = "objectnumber")]
//     pub object_number: String,
//     pub lghnummer: String,
//     pub address: String,
//     pub zipcode: String,
//     pub city: String,
//     #[serde(with = "as_str")]
//     pub floor: i8,
//     #[serde(with = "as_str")]
//     pub sqr_mtrs: f32,
//     #[serde(with = "as_str")]
//     pub reserved: bool,
//     #[serde(with = "as_str")]
//     pub number_of_reservations: u32,
//     #[serde(with = "as_str")]
//     pub queue_number: u32,
//     #[serde(with = "yyyy_mm_dd")]
//     pub move_in_date: Date,
//     #[serde(with = "yyyy_mm_dd")]
//     pub reserve_from_date: Date,
//     #[serde(with = "yyyy_mm_dd")]
//     pub reserve_until_date: Date,
//     #[serde(with = "as_str")]
//     pub rent: u32,
// }

pub type Product = serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum ListVacantError {
    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("error returned from api: {0}")]
    ApiError(String),
}

pub struct Client {
    inner: ClientWithMiddleware,
    email: String,
    password: SecretString,
}

impl Client {
    pub fn new(email: impl Into<String>, password: impl Into<SecretString>) -> Self {
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
            email: email.into(),
            password: password.into(),
        }
    }

    pub async fn list_vacant(&self) -> Result<Vec<Product>, ListVacantError> {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum Response {
            Product {
                product: Vec<Product>,
            },
            Error {
                error: String,
                message: Option<String>,
            },
        }

        match self
            .inner
            .get("https://diremoapi.afbostader.se/redimo/rest/vacantproducts?lang=sv_SE&type=1")
            .basic_auth(&self.email, Some(self.password.expose_secret()))
            .send()
            .await?
            .json::<Response>()
            .await?
        {
            Response::Product { product } => Ok(product),
            Response::Error { error, message } => {
                Err(ListVacantError::ApiError(message.unwrap_or(error)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Product;

    #[test]
    fn parse_product() {
        let json = include_bytes!("product.json");
        let _: Product = serde_json::from_slice(json).unwrap();
    }
}
