use axum::{Router, routing::get};
use dotenv::dotenv;

use crate::people::get_people;

mod conn;
mod model;
mod people;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/people", get(get_people));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
