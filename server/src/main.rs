use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, extract::{Query, State}, response::{IntoResponse, Response}, routing::get, Router};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::{
    buffer::BufferLayer, limit::{RateLimitLayer}, BoxError, ServiceBuilder
};
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize)]
struct GeocodeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
}

#[derive(Debug, thiserror::Error)]
enum GeocodeError {
    #[error(transparent)]
    Http(#[from] reqwest::Error)
}

impl IntoResponse for GeocodeError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn geocode(State(state): State<AppState>, Query(query): Query<GeocodeQuery>) -> Result<impl IntoResponse, GeocodeError> {
    println!("handling {query:?}");

    let res = state.client.get("https://nominatim.openstreetmap.org/search.php?format=jsonv2").query(&query).send().await?;
    Ok(([("content-type", "application/json")], res.text().await?))
}

#[derive(Debug, Clone)]
struct AppState {
    client: Client,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/geocode",
            get(geocode).route_layer(ServiceBuilder::new()),
        )
        .layer(ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled error: {}", err),
            )
        }))
        .layer(BufferLayer::new(1024))
        .layer(RateLimitLayer::new(1, Duration::from_secs(1))),)
        .layer(CorsLayer::permissive())
        .with_state(AppState {
            client: Client::builder()
                .user_agent("afbostader-api")
                .build()
                .unwrap(),
        });
    let listener = TcpListener::bind("[::]:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
