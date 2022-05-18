use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Option<SqliteConnection> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("$DATABASE_URL must be set!");

    Some(
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
    )
}
