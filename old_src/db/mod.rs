use diesel::prelude::*;
use dotenv::dotenv;
use std::env::{self, VarError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum DbError {
    #[error("Error connecting to SQLite database.")]
    SqliteConnectionFailure(#[source] ConnectionError),
    #[error("Error getting env vars.")]
    EnvVarMissing(#[source] VarError),
    #[error("Error with `.env`. Is it created?")]
    DotEnvMissing,
}

pub(crate) type DbResult<T, E = DbError> = anyhow::Result<T, E>;

pub(crate) fn establish_connection() -> DbResult<SqliteConnection> {
    dotenv().ok().ok_or(DbError::DotEnvMissing)?;

    let url = env::var("DATABASE_URL").map_err(DbError::EnvVarMissing)?;

    SqliteConnection::establish(&url).map_err(DbError::SqliteConnectionFailure)
}
