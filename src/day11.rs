use std::io::Cursor;

use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, routing::post, Router};
use image::{io::Reader, DynamicImage};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(task11))
}

async fn task11(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() == "image" {
            let data = field.bytes().await.unwrap();
            let image = Reader::new(Cursor::new(data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            return num_magical_red(image).to_string().into_response();
        }
    }
    (StatusCode::NOT_FOUND).into_response()
}

fn num_magical_red(img: DynamicImage) -> usize {
    img.as_bytes()
        .chunks(3)
        .filter(|p| p[0] > p[1].saturating_add(p[2]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_red() {
        let img = image::open("assets/decoration.png").unwrap();
        let output = num_magical_red(img);
        let expected = 73034;

        assert_eq!(output, expected);
    }
}
