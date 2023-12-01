use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Ho, ho, ho!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*nums", get(task1));

    Ok(router.into())
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
