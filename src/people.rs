use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::Person;

#[derive(Deserialize, Serialize)]
pub struct People {
    people: Vec<Person>,
    total_people: i32,
}

pub async fn get_people() -> Json<People> {
    let people = People {
        people: Vec::new(),
        total_people: 0,
    };

    Json(people)
}
