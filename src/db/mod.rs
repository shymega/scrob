use diesel::prelude::*;
use dotenv::dotenv;
use std::env::{self, VarError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum DbInitialisationError {
    #[error("Error connecting to SQLite database.")]
    SqliteConnectionError(#[source] ConnectionError),
    #[error("Error getting env vars.")]
    EnvVarError(#[source] VarError),
    #[error("Error with `.env`. Is it created?")]
    DotEnvGetError,
}

pub(crate) fn establish_connection(
) -> Result<SqliteConnection, DbInitialisationError> {
    dotenv().ok()
        .ok_or(DbInitialisationError::DotEnvGetError)?;

    let url = env::var("DATABASE_URL")
        .map_err(DbInitialisationError::EnvVarError)?;

    Ok(SqliteConnection::establish(&url)
        .map_err(DbInitialisationError::SqliteConnectionError)?)
}
