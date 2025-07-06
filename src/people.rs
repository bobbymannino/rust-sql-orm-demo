use axum::Json;
use axum::http::StatusCode;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;

use crate::conn::get_connection;
use crate::model::Person;
use crate::schema::person::dsl::person;

pub struct ErrorType {
    message: String,
    status: u16,
}

type ApiError = (StatusCode, Json<ErrorType>);
type ApiData<T> = (StatusCode, Json<T>);
type ApiResponse<T> = Result<ApiData<T>, ApiError>;

#[derive(Serialize)]
pub struct PeopleResponse {
    people: Vec<Person>,
    total_people: usize,
}

pub async fn get_people() -> ApiResponse<PeopleResponse> {
    let people = get_people_from_db().await;

    match people {
        Ok(people) => {
            let response = PeopleResponse {
                total_people: people.len(),
                people,
            };

            Ok((StatusCode::OK, Json(response)))
        }
        Err(_) => {
            let error = ErrorType {
                message: "Failed to get people".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

async fn get_people_from_db() -> Result<Vec<Person>, diesel::result::Error> {
    let conn = &mut get_connection();

    person.select(Person::as_select()).load(conn)
}
