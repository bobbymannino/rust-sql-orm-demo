use diesel::{data_types::PgTimestamp, prelude::*};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::person)]
#[diesel(primary_key(person_id))]
pub struct Person {
    pub person_id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: PgTimestamp,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Associations)]
#[diesel(belongs_to(Person, foreign_key = author_id))]
#[diesel(table_name = crate::schema::post)]
#[diesel(primary_key(post_id))]
pub struct Post {
    pub post_id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: PgTimestamp,
}
