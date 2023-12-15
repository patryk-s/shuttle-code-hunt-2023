use axum::{extract::State, response::IntoResponse, routing::get, Router};
use sqlx::PgPool;

pub fn router(pool: PgPool) -> Router {
    Router::new().nest(
        "/13",
        Router::new().route("/sql", get(task13)).with_state(pool),
    )
}

async fn task13(State(db): State<PgPool>) -> impl IntoResponse {
    let res: i32 = sqlx::query_scalar!(r#"SELECT 20231213"#)
        .fetch_one(&db)
        .await
        .unwrap()
        .unwrap();
    res.to_string()
}
