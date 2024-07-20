use std::{sync::Arc, time::Duration};

use archive_af::read::QueueHistoryQuery;
use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use axum_extra::TypedHeader;
use clap::Parser;
use client_af::{Credentials, PropertyId};
use git2::Repository;
use headers::CacheControl;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};
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

async fn list_vacancies(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.af.list_vacancies().await.unwrap())
}

async fn get_vacancy_detail(
    State(state): State<AppState>,
    Path(id): Path<PropertyId>,
) -> impl IntoResponse {
    Json(state.af.vacancy_detail(id).await.unwrap())
}

async fn get_area_detail(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let detail = state.af.area_detail(&name).await.unwrap();
    (
        TypedHeader(
            CacheControl::new()
                .with_public()
                .with_max_age(Duration::from_secs(3600)),
        ),
        Json(detail),
    )
}

#[derive(Debug, thiserror::Error)]
enum GetArchiveDataError {
    #[error(transparent)]
    Git(#[from] git2::Error),
}

impl IntoResponse for GetArchiveDataError {
    fn into_response(self) -> Response {
        todo!()
    }
}

async fn get_archive_data(
    Query(query): Query<QueueHistoryQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, GetArchiveDataError> {
    let repo = state.repo.lock().await;
    if archive_af::git::pull(&repo).unwrap().status.success() {
        tracing::info!("pulled git repo");
    }

    let chart = archive_af::read::queue_history(&repo, &query)?;

    Ok(Json(chart))
}

#[derive(Clone)]
struct AppState {
    af: client_af::Client,
    client: reqwest::Client,
    repo: Arc<Mutex<Repository>>,
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, env)]
    email: String,
    #[clap(long, env)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let Args { email, password } = Args::parse();

    let af = client_af::Client::new().with_credentials(Credentials::new(email, password));

    let app = Router::new()
        .route("/vacancies", get(list_vacancies))
        .route("/vacancies/:id", get(get_vacancy_detail))
        .route("/areas/:name", get(get_area_detail))
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
        .route("/archive", get(get_archive_data))
        .layer(CorsLayer::permissive())
        .with_state(AppState {
            client: reqwest::Client::builder()
                .user_agent("afbostader-api")
                .build()
                .unwrap(),
            af,
            repo: Arc::new(Mutex::new(Repository::open("af-data")?)),
        });
    let listener = TcpListener::bind("[::]:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
