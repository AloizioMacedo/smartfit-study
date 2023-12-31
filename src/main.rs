use aloizio_smartfit::{
    get_results::get_results,
    location::{Data, Loc},
};
use axum::{response::Html, routing::get, Json, Router};
use tower_http::services::{ServeDir, ServeFile};

async fn get_locations() -> Json<Vec<Loc>> {
    let file = std::fs::read_to_string("locations.json").expect("JSON file should be accessible");

    let data: Data = serde_json::from_str(&file).unwrap();

    Json(data.locations)
}

async fn clean() -> Html<&'static str> {
    Html(
        r#"<strong class="number-of-results"
        id="number-of-results" hx-swap-oob="true">0</strong>"#,
    )
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .nest_service("/images", ServeDir::new("templates/images"))
        .nest_service("/style.css", ServeFile::new("templates/style.css"))
        .nest_service("/fonts", ServeDir::new("templates/fonts"))
        .route("/", get(index))
        .route("/locations", get(get_locations))
        .route("/results", get(get_results))
        .route("/clean", get(clean));

    Ok(router.into())
}

async fn index() -> Html<String> {
    let file = std::fs::read_to_string("templates/index.html")
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
