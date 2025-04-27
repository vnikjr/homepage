use rocket::Route;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::{get, routes};

pub fn get_routes() -> Vec<Route> {
    routes![snake, snakejs]
}

#[get("/snake_bg.wasm")]
async fn snake() -> Option<(ContentType, NamedFile)> {
    NamedFile::open("static/wasm/snake/snake_bg.wasm")
        .await
        .ok()
        .map(|file| (ContentType::new("application", "wasm"), file))
}

#[get("/snake.js")]
async fn snakejs() -> Option<(ContentType, NamedFile)> {
    NamedFile::open("static/wasm/snake/snake.js")
        .await
        .ok()
        .map(|file| (ContentType::new("application", "javascript"), file))
}
