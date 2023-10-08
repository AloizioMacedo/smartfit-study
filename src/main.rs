use axum::{response::Html, routing::get, Json, Router};
use smartfit::{
    get_results::get_results,
    location::{Data, Loc},
};
use tower_http::services::{ServeDir, ServeFile};

async fn get_locations() -> Json<Vec<Loc>> {
    let file = std::fs::read_to_string("locations.json").expect("JSON file should be accessible");

    let data: Data = serde_json::from_str(&file).unwrap();

    Json(data.locations)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest_service("/images", ServeDir::new("images"))
        .nest_service("/style.css", ServeFile::new("templates/style.css"))
        .route("/", get(index))
        .route("/locations", get(get_locations))
        .route("/results", get(get_results));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    let file = std::fs::read_to_string("templates/template.html")
        .expect("Should be able to get template.html");

    Html(file)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_chrono() {
        let x = chrono::NaiveTime::parse_from_str("06h30", "%Hh%M").unwrap();
        let y = chrono::NaiveTime::parse_from_str("06h30", "%Hh%M").unwrap();
        println!("{:?}", x >= y);
    }
}
