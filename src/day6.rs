use axum::{response::IntoResponse, routing::post, Json, Router};
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/6", post(task6))
}

async fn task6(body: String) -> impl IntoResponse {
    let elf_count = parse_elf(&body);
    let shelf_count = parse_shelves(&body);
    let no_elf_count = body.matches("shelf").count() - shelf_count;
    Json(json!({
        "elf": elf_count,
        "elf on a shelf": shelf_count,
        "shelf with no elf on it": no_elf_count,
    }))
}

fn parse_elf(input: &str) -> usize {
    input.matches("elf").count()
}

fn parse_shelves(input: &str) -> usize {
    let shelves = "elf on a shelf";
    input
        .as_bytes()
        .windows(shelves.len())
        .filter(|s| std::str::from_utf8(s).unwrap() == shelves)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "In Belfast I heard an elf on a shelf on a shelf on a";

    #[test]
    fn elf_count() {
        let output = parse_elf(INPUT);
        assert_eq!(output, 4);
    }

    #[test]
    fn shelves_count() {
        let output = parse_shelves(INPUT);
        assert_eq!(output, 2);
    }
}
