use diesel::prelude::*;
use dotenv::dotenv;
use std::env::{self, VarError};
use std::error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub(crate) enum DbInitialisationError {
    SqliteConnectionError(ConnectionError),
    EnvVarError(VarError),
    DotEnvGetError,
}

impl error::Error for DbInitialisationError {}

impl fmt::Display for DbInitialisationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &*self {
            DbInitialisationError::DotEnvGetError => {
                write!(f, "Error with `dotenv`. Missing `.env`?")
            }
            DbInitialisationError::EnvVarError(e) => {
                write!(
                    f,
                    "Error getting env variables - error: {}",
                    e.to_string()
                )
            }
            DbInitialisationError::SqliteConnectionError(e) => {
                write!(
                    f,
                    "Error connecting to SQLite DB - error: {}",
                    e.to_string()
                )
            }
        }
    }
}

impl From<ConnectionError> for DbInitialisationError {
    fn from(e: ConnectionError) -> Self {
        Self::SqliteConnectionError(e)
    }
}

impl From<VarError> for DbInitialisationError {
    fn from(e: VarError) -> Self {
        Self::EnvVarError(e)
    }
}

#[allow(dead_code)]
pub(crate) fn establish_connection(
) -> Result<SqliteConnection, DbInitialisationError> {
    match dotenv().ok() {
        Some(_) => (),
        None => return Err(DbInitialisationError::DotEnvGetError),
    };

    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => return Err(DbInitialisationError::EnvVarError(e)),
    };

    match SqliteConnection::establish(&db_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(DbInitialisationError::SqliteConnectionError(e)),
    }
}
