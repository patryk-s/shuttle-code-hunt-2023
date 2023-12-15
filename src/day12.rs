use std::{collections::BTreeMap, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Datelike, Utc, Weekday};
use serde_json::json;
use tokio::{sync::Mutex, time::Instant};
use ulid::Ulid;
use uuid::Uuid;

pub type Timekeeper = Arc<Mutex<BTreeMap<String, Instant>>>;

pub fn router(db: Timekeeper) -> Router {
    Router::new().nest(
        "/12",
        Router::new()
            .route("/save/:text", post(task12_save))
            .route("/load/:text", get(task12_load))
            .with_state(db.clone())
            .route("/ulids", post(task12_ulid))
            .route("/ulids/:weekday", post(task12_weekday)),
    )
}

async fn task12_weekday(
    Path(weekday): Path<u8>,
    Json(ulids): Json<Vec<String>>,
) -> impl IntoResponse {
    let weekday = Weekday::try_from(weekday).unwrap();
    let ulids: Vec<Ulid> = ulids
        .iter()
        .map(|s| Ulid::from_string(s).unwrap())
        .collect();

    let mut christmas = 0_u32;
    let mut weekday_count = 0_u32;
    let mut in_future = 0_u32;
    let mut lsb = 0_u32;
    let now = Utc::now();

    for u in ulids.into_iter() {
        let utime: DateTime<Utc> = u.datetime().into();
        if utime > now {
            in_future += 1;
        }
        if utime.weekday() == weekday {
            weekday_count += 1;
        }
        if utime.day() == 24 && utime.month() == 12 {
            christmas += 1;
        }
        if u.random() % 2 != 0 {
            lsb += 1;
        }
    }

    Json(json!({
        "christmas eve": christmas,
        "weekday": weekday_count,
        "in the future": in_future,
        "LSB is 1": lsb,
    }))
}

fn ulid2uuid(ulid: String) -> Uuid {
    Ulid::from_string(&ulid).unwrap().into()
}

async fn task12_ulid(Json(ulids): Json<Vec<String>>) -> impl IntoResponse {
    let mut uuids: Vec<Uuid> = ulids.into_iter().map(ulid2uuid).collect();
    uuids.reverse();
    Json(uuids)
}

async fn task12_load(State(db): State<Timekeeper>, Path(text): Path<String>) -> impl IntoResponse {
    match db.lock().await.get(&text) {
        Some(time) => time.elapsed().as_secs().to_string().into_response(),
        None => (StatusCode::NOT_FOUND, "No such string").into_response(),
    }
}

async fn task12_save(State(db): State<Timekeeper>, Path(text): Path<String>) -> impl IntoResponse {
    let time = Instant::now();
    db.lock_owned()
        .await
        .entry(text)
        .and_modify(|t| *t = time)
        .or_insert(time);
}
