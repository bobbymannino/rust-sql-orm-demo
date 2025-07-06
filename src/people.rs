use axum::Json;
use axum::extract::Json as ExtractJson;
use axum::http::StatusCode;
use diesel::result::Error as DieselError;
use diesel::{insert_into, prelude::*};
use serde::Serialize;

use crate::conn::get_connection;
use crate::model::{CreatePerson, Person};
use crate::schema::person::dsl::person;

#[derive(Serialize)]
pub struct ApiError {
    message: String,
}

pub enum ApiResponse<T> {
    Error((StatusCode, Json<ApiError>)),
    Success((StatusCode, Json<T>)),
}

impl<T> axum::response::IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Error(res) => res.into_response(),
            ApiResponse::Success(res) => res.into_response(),
        }
    }
}

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

            ApiResponse::Success((StatusCode::OK, Json(response)))
        }
        Err(_) => {
            let error = ApiError {
                message: "Failed to get people".to_string(),
            };

            ApiResponse::Error((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

async fn get_people_from_db() -> Result<Vec<Person>, DieselError> {
    let conn = &mut get_connection();

    person.select(Person::as_select()).load(conn)
}

pub async fn create_person(
    ExtractJson(new_person): ExtractJson<CreatePerson>,
) -> ApiResponse<Person> {
    if new_person.last_name.trim().is_empty() {
        let error = ApiError {
            message: "'last_name' cannot be empty".to_string(),
        };

        return ApiResponse::Error((StatusCode::BAD_GATEWAY, Json(error)));
    }

    let new_person = create_person_in_db(new_person);

    match new_person {
        Ok(new_person) => ApiResponse::Success((StatusCode::CREATED, Json(new_person))),
        Err(_) => {
            let error = ApiError {
                message: "Failed to create person".to_string(),
            };

            ApiResponse::Error((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

fn create_person_in_db(new_person: CreatePerson) -> Result<Person, DieselError> {
    let conn = &mut get_connection();

    insert_into(crate::schema::person::table)
        .values(&new_person)
        .returning(Person::as_returning())
        .get_result(conn)
}
