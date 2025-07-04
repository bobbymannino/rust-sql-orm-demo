use axum::Json;
use axum::http::StatusCode;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::conn::get_connection;
use crate::model::Person;
use crate::schema::person::dsl::person;

#[derive(Deserialize, Serialize)]
pub struct People {
    people: Vec<Person>,
    total_people: i32,
}

pub async fn get_people() -> Result<Json<People>, StatusCode> {
    let people = get_people_from_db().await;

    match people {
        Ok(people) => Ok(Json(People {
            total_people: people.len() as i32,
            people,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_people_from_db() -> Result<Vec<Person>, Error> {
    let conn = &mut get_connection();

    person.select(Person::as_select()).load(conn)
}
