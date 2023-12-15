use axum::{
    response::{Html, IntoResponse},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/4/strength", post(task4))
        .route("/4/contest", post(task4_contest))
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
