use askama::Template;
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new().nest(
        "/14",
        Router::new()
            .route("/unsafe", post(task14_unsafe))
            .route("/safe", post(task14_safe)),
    )
}

#[derive(Deserialize, Serialize, Template)]
#[template(path = "task14.html")]
struct Content {
    content: String,
    not_safe: Option<bool>,
}

async fn task14_unsafe(Json(data): Json<Content>) -> impl IntoResponse {
    let data = Content {
        not_safe: Some(true),
        ..data
    };
    data.into_response()
}

async fn task14_safe(Json(data): Json<Content>) -> impl IntoResponse {
    data.into_response()
}
