use std::{convert::Infallible, ops::Deref};

use afbostader::Credentials;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::{cookie::Key, PrivateCookieJar};
use serde::{Deserialize, Serialize};

pub mod floorplan;

#[derive(Clone)]
pub struct AppState {
    pub af: afbostader::Client,
    pub client: reqwest::Client,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(input: &AppState) -> Self {
        input.key.clone()
    }
}

pub struct PersonalAf(pub afbostader::Client);

impl Deref for PersonalAf {
    type Target = afbostader::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmailPassword {
    email: String,
    password: String,
}

impl From<EmailPassword> for Credentials {
    fn from(EmailPassword { email, password }: EmailPassword) -> Self {
        Self::new(email, password)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for PersonalAf {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .unwrap();
        let client = state.af.clone();

        if let Some(details) = jar
            .get("login")
            .and_then(|c| serde_json::from_str::<EmailPassword>(c.value()).ok())
        {
            Ok(PersonalAf(client.with_credentials(details.into())))
        } else {
            Ok(PersonalAf(client))
        }
    }
}
