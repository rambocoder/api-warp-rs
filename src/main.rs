use dotenv;
use warp::{http::StatusCode, Filter};

mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = db::create_pool(&database_url);

    let health_route = warp::path("health").map(|| StatusCode::OK);

    let hello_route = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let routes = health_route
        .or(hello_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
