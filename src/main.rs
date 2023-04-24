use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Input {
    a: f64,
    b: f64,
}

#[derive(Deserialize, Serialize)]
struct Output {
    result: f64,
}

async fn minus(input: Input) -> Result<impl warp::Reply, warp::Rejection> {
    let result = input.a - input.b;
    let output = Output { result };
    Ok(warp::reply::json(&output))
}

async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Minus server is running!", warp::http::StatusCode::OK))
}

#[tokio::main]
async fn main() {
    let minus_route = warp::post()
        .and(warp::path("minus"))
        .and(warp::body::json())
        .and_then(minus);

    let health_route = warp::get()
        .and(warp::path("health"))
        .and_then(health_check);

    let routes = minus_route.or(health_route);

    println!("Minus server started at localhost:5012");

    warp::serve(routes).run(([127, 0, 0, 1], 5012)).await;

}