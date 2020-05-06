// manual boilerplate is here is to provide an example.
// for production an error macro crate can be used

#[derive(Debug)]
pub enum Error {
    LoadConfig(envy::Error),
    ParseAddress(std::net::AddrParseError),
    Transport(tonic::transport::Error),
    Database(sqlx::Error),
}

use Error::*;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadConfig(err) => err.fmt(f),
            Transport(err) => err.fmt(f),
            ParseAddress(err) => err.fmt(f),
            Database(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<envy::Error> for Error {
    fn from(e: envy::Error) -> Self {
        LoadConfig(e)
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        Transport(e)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Self {
        ParseAddress(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Database(e)
    }
}
