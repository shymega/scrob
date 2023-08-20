use diesel::prelude::*;
use dotenv::dotenv;
use std::env::{self, VarError};
use std::error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, thiserror::Error)]
pub(crate) enum DbInitialisationError {
    #[error("Error connecting to SQLite database.")]
    SqliteConnectionError(#[source] ConnectionError),
    #[error("Error getting env vars.")]
    EnvVarError(#[source] VarError),
    #[error("Error with `.env`. Is it created?")]
    DotEnvGetError(#[source] dotenv::Error)
}

pub(crate) fn establish_connection(
) -> Result<SqliteConnection, DbInitialisationError> {
    dotenv()
        .ok()
        .ok_or(DbInitialisationError::DotEnvGetError)?;

    let url = env::var("DATABASE_URL")
        .ok()
        .ok_or(DbInitialisationError::EnvVarError)?;


    SqliteConnection::establish(&url)
        .ok()
        .ok_or(DbInitialisationError::SqliteConnectionError)?;
}
