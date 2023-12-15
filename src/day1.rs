use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(task_error))
        .route("/1/*nums", get(task1))
}

async fn task1(Path(params): Path<String>) -> impl IntoResponse {
    Html(
        (params
            .split('/')
            .map(|s| s.parse::<i64>().unwrap())
            .fold(0, |all, x| all ^ x))
        .pow(3)
        .to_string(),
    )
}

async fn hello_world() -> &'static str {
    "Ho, ho, ho!"
}

async fn task_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}
