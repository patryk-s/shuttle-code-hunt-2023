mod day1;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day4;
mod day6;
mod day7;
mod day8;

use std::collections::BTreeMap;

use axum::Router;
use sqlx::PgPool;
use tokio::sync::Mutex;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize in-memory database
    let db = day12::Timekeeper::new(Mutex::new(BTreeMap::new()));

    let router = Router::new()
        .merge(day1::router())
        .merge(day4::router())
        .merge(day6::router())
        .merge(day7::router())
        .merge(day8::router())
        .merge(day11::router())
        .merge(day12::router(db))
        .merge(day13::router(pool))
        .merge(day15::router())
        .merge(day14::router());

    Ok(router.into())
}
