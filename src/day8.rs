use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use rustemon::{client::RustemonClient, pokemon::pokemon};

pub fn router() -> Router {
    Router::new()
        .route("/8/weight/:id", get(task8))
        .route("/8/drop/:id", get(task8_part2))
}

#[axum::debug_handler]
async fn task8_part2(Path(id): Path<i64>) -> impl IntoResponse {
    let rustemon_client = RustemonClient::default();
    const G: f64 = 9.825;
    let height = 10.0;
    let velocity_i = (2.0 * G * height).sqrt();
    match pokemon::get_by_id(id, &rustemon_client).await {
        Ok(pokemon) => Ok((pokemon.weight as f64 / 10.0 * velocity_i).to_string()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[axum::debug_handler]
async fn task8(Path(id): Path<i64>) -> impl IntoResponse {
    let rustemon_client = RustemonClient::default();
    match pokemon::get_by_id(id, &rustemon_client).await {
        Ok(pokemon) => Ok((pokemon.weight as f64 / 10.0).to_string()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
