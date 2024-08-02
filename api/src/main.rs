use std::{io::Cursor, time::Duration};

use afbostader::PropertyId;
use amcoff_bostader_api::{floorplan::{self, ToImageError}, AppState, EmailPassword, PersonalAf};
use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::{
    extract::{
        cookie::{Cookie, Key, SameSite},
        PrivateCookieJar,
    },
    TypedHeader,
};
use clap::Parser;
use headers::{CacheControl, ContentType};
use image::ImageFormat;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize)]
struct GeocodeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postalcode: Option<String>,
}

#[derive(Debug, thiserror::Error)]
enum GeocodeError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

impl IntoResponse for GeocodeError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn geocode(
    State(state): State<AppState>,
    Query(query): Query<GeocodeQuery>,
) -> Result<impl IntoResponse, GeocodeError> {
    let res = state
        .client
        .get("https://nominatim.openstreetmap.org/search.php?format=jsonv2")
        .query(&query)
        .send()
        .await?;
    Ok(([("content-type", "application/json")], res.text().await?))
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct AfError(#[from] afbostader::Error);

impl AfError {
    fn status_code(&self) -> StatusCode {
        use afbostader::Error;

        match self.0 {
            Error::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadCredentials => StatusCode::FORBIDDEN,
            Error::Unauthenticated => StatusCode::UNAUTHORIZED,
            Error::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AfError {
    fn into_response(self) -> Response {
        (self.status_code(), self.to_string()).into_response()
    }
}

async fn list_vacancies(af: PersonalAf) -> Result<impl IntoResponse, AfError> {
    Ok((
        TypedHeader(CacheControl::new().with_private()),
        Json(af.list_vacancies().await?),
    ))
}

async fn get_vacancy_detail(
    af: PersonalAf,
    Path(id): Path<PropertyId>,
) -> Result<impl IntoResponse, AfError> {
    Ok((
        TypedHeader(CacheControl::new().with_private()),
        Json(af.0.vacancy_detail(id).await?),
    ))
}

#[derive(Debug, thiserror::Error)]
enum FloorplanError {
    #[error("http error")]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    ToImageError(#[from] ToImageError),
    #[error(transparent)]
    Af(AfError),
}

impl<T> From<T> for FloorplanError
where
    AfError: From<T>,
{
    fn from(value: T) -> Self {
        Self::Af(AfError::from(value))
    }
}

impl IntoResponse for FloorplanError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn get_vacancy_floorplan(
    State(state): State<AppState>,
    Path(id): Path<PropertyId>,
) -> Result<Response, FloorplanError> {
    let Some(url) = state
        .af
        .vacancy_detail(id)
        .await
        .map_err(AfError)?
        .blueprint
    else {
        return Ok(StatusCode::NO_CONTENT.into_response());
    };

    let res = state.client.get(url).send().await?;

    let img = floorplan::to_image(res).await?;

    let png = tokio::task::spawn_blocking(move || {
        let mut out = Cursor::new(Vec::new());
        img.write_to(&mut out, ImageFormat::Png).unwrap();
        out.into_inner()
    })
    .await
    .unwrap();

    Ok((
        TypedHeader(ContentType::png()),
        TypedHeader(CacheControl::new().with_max_age(Duration::from_secs(86_400))),
        png,
    )
        .into_response())
}

async fn get_area_detail(
    af: PersonalAf,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, AfError> {
    let detail = af.0.area_detail(&name).await?;
    Ok((
        TypedHeader(
            CacheControl::new()
                .with_public()
                .with_max_age(Duration::from_secs(3600)),
        ),
        Json(detail),
    ))
}

async fn login(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(details): Json<EmailPassword>,
) -> Result<impl IntoResponse, AfError> {
    let cookie = Cookie::build(("login", serde_json::to_string(&details).unwrap()))
        .http_only(true)
        .path("/")
        .permanent()
        .secure(false)
        .same_site(SameSite::None)
        .build();

    let user = state
        .af
        .with_credentials(details.into())
        .user_info()
        .await?;

    Ok((jar.add(cookie), Json(user)))
}

async fn user(af: PersonalAf) -> Result<impl IntoResponse, AfError> {
    Ok((
        TypedHeader(CacheControl::new().with_no_cache()),
        Json(af.user_info().await?),
    ))
}

async fn logout(jar: PrivateCookieJar) -> impl IntoResponse {
    jar.remove("login")
}

#[derive(Debug, Parser)]
struct Args {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let Args {} = Args::parse();

    let af = afbostader::Client::new();

    let app = Router::new()
        .route("/vacancies", get(list_vacancies))
        .route("/vacancies/:id", get(get_vacancy_detail))
        .route("/vacancies/:id/floorplan", get(get_vacancy_floorplan))
        .route("/areas/:name", get(get_area_detail))
        .route("/login", post(login))
        .route("/user", get(user))
        .route("/logout", get(logout))
        .route(
            "/geocode",
            get(geocode).route_layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(|err: BoxError| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled error: {}", err),
                        )
                    }))
                    .layer(BufferLayer::new(1024))
                    // The "absolute maximum" according to the Nominatim Usage Policy
                    // is 1 request per second
                    .layer(RateLimitLayer::new(1, Duration::from_secs(1))),
            ),
        )
        .layer(CorsLayer::very_permissive())
        .with_state(AppState {
            client: reqwest::Client::builder()
                .user_agent(afbostader::USER_AGENT)
                .build()
                .unwrap(),
            af,
            key: Key::generate(),
        });
    let listener = TcpListener::bind("[::]:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
