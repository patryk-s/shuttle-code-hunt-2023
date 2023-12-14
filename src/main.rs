use std::{collections::BTreeMap, io::Cursor, sync::Arc};

use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Datelike, Utc, Weekday};
use image::{io::Reader, DynamicImage};
use rustemon::{client::RustemonClient, pokemon::pokemon};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tokio::{sync::Mutex, time::Instant};
use tower_http::services::ServeDir;
use ulid::Ulid;
use uuid::Uuid;

async fn hello_world() -> &'static str {
    "Ho, ho, ho!"
}

type Timekeeper = Arc<Mutex<BTreeMap<String, Instant>>>;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize in-memory database
    let db = Timekeeper::new(Mutex::new(BTreeMap::new()));

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(task_error))
        .route("/1/*nums", get(task1))
        .route("/4/strength", post(task4))
        .route("/4/contest", post(task4_contest))
        .route("/6", post(task6))
        .route("/7/decode", get(task7))
        .route("/8/weight/:id", get(task8))
        .route("/8/drop/:id", get(task8_part2))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(task11))
        .nest(
            "/12",
            Router::new()
                .route("/save/:text", post(task12_save))
                .route("/load/:text", get(task12_load))
                .with_state(db.clone())
                .route("/ulids", post(task12_ulid))
                .route("/ulids/:weekday", post(task12_weekday)),
        )
        .nest(
            "/13",
            Router::new().route("/sql", get(task13)).with_state(pool),
        )
        .nest(
            "/14",
            Router::new()
                .route("/unsafe", post(task14_unsafe))
                .route("/safe", post(task14_safe)),
        );

    Ok(router.into())
}

#[derive(Deserialize, Serialize, Template)]
#[template(path = "task14.html")]
struct Content {
    content: String,
    not_safe: Option<bool>,
}

async fn task14_unsafe(Json(data): Json<Content>) -> impl IntoResponse {
    let data = Content {
        not_safe: Some(true),
        ..data
    };
    data.into_response()
}

async fn task14_safe(Json(data): Json<Content>) -> impl IntoResponse {
    data.into_response()
}

async fn task13(State(db): State<PgPool>) -> impl IntoResponse {
    let res: i32 = sqlx::query_scalar!(r#"SELECT 20231213"#)
        .fetch_one(&db)
        .await
        .unwrap()
        .unwrap();
    res.to_string()
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Reindeer1 {
    name: String,
    strength: i32,
}

impl std::iter::Sum<Reindeer1> for i32 {
    fn sum<I: Iterator<Item = Reindeer1>>(iter: I) -> Self {
        iter.fold(0, |acc, s| acc + s.strength)
    }
}

#[derive(Debug, Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: i32,
}

#[derive(Debug, Serialize)]
struct Results {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn task4_contest(Json(input): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let first = input.first().unwrap();
    let mut fastest = first;
    let mut tallest = first;
    let mut magician = first;
    let mut consumer = first;

    for s in &input {
        if s.speed > fastest.speed {
            fastest = s
        }
        if s.height > tallest.height {
            tallest = s
        }
        if s.snow_magic_power > magician.snow_magic_power {
            magician = s
        }
        if s.candies_eaten_yesterday > consumer.candies_eaten_yesterday {
            consumer = s
        }
    }

    Json(Results {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}

async fn task4(Json(input): Json<Vec<Reindeer1>>) -> impl IntoResponse {
    let sum: i32 = input.into_iter().sum();
    Html(sum.to_string())
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
async fn task_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
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

    #[test]
    fn magic_red() {
        let img = image::open("assets/decoration.png").unwrap();
        let output = num_magical_red(img);
        let expected = 73034;

        assert_eq!(output, expected);
    }
}
