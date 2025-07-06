mod conn;
mod model;
mod people;
mod schema;

use axum::{Router, routing::get};
use dotenv::dotenv;

use crate::people::{create_person, get_people};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/people", get(get_people).post(create_person));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();

    println!("Listening on port 3005");

    axum::serve(listener, app).await.unwrap();
}
