use std::env;

use diesel::{Connection, PgConnection};

pub fn get_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to connect to {}", database_url))
}
