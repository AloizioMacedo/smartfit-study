use axum::{routing::get, Json, Router};
use smartfit::location::{Data, Loc};

async fn get_locations() -> Json<Vec<Loc>> {
    let file = std::fs::read_to_string("locations.json").expect("JSON file should be accessible");

    let data: Data = serde_json::from_str(&file).unwrap();

    Json(data.locations)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/locations", get(get_locations));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
