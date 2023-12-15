use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/15/nice", post(task15_part1))
}

#[derive(Deserialize, Serialize)]
struct Input {
    input: String,
}

async fn task15_part1(Json(req): Json<Input>) -> impl IntoResponse {
    let mut has_bad_string = false;
    let bad_strings = ["ab", "cd", "pq", "xy"];
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for bad in bad_strings {
        if req.input.contains(bad) {
            has_bad_string = true;
            break;
        }
    }

    let vowel_count = req.input.chars().filter(|c| vowels.contains(c)).count();

    if !has_bad_string && vowel_count >= 3 {
        Json(json!({"result": "nice"})).into_response()
    } else {
        (StatusCode::BAD_REQUEST, Json(json!({"result": "naughty"}))).into_response()
    }
}
