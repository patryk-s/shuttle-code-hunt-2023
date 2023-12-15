use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use base64::{engine::general_purpose, Engine as _};

pub fn router() -> Router {
    Router::new().route("/7/decode", get(task7))
}

async fn task7(jar: CookieJar) -> impl IntoResponse {
    if let Some(recipe) = jar.get("recipe") {
        let data = recipe.value();
        let data = general_purpose::STANDARD.decode(data).unwrap();
        let data = String::from_utf8(data).unwrap();
        println!("{data}");
        Html(data)
    } else {
        Html("Not found".to_string())
    }
}
