use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::person)]
#[diesel(primary_key(person_id))]
pub struct Person {
    pub person_id: i32,
    pub first_name: Option<String>,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::person)]
pub struct CreatePerson {
    pub first_name: Option<String>,
    pub last_name: String,
}

#[derive(
    Serialize, Deserialize, Queryable, Identifiable, Selectable, Debug, PartialEq, Associations,
)]
#[diesel(belongs_to(Person, foreign_key = author_id))]
#[diesel(table_name = crate::schema::post)]
#[diesel(primary_key(post_id))]
pub struct Post {
    pub post_id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
}
