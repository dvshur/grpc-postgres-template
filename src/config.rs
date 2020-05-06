use crate::error::*;
use serde::Deserialize;

fn default_port() -> u16 {
    5432
}

fn default_pool_size() -> u32 {
    4
}

#[derive(Deserialize, Debug)]
pub struct PgConfig {
    #[serde(rename = "PGPOOL", default = "default_pool_size")]
    pub pool_size: u32,
    #[serde(rename = "PGHOST")]
    pub host: String,
    #[serde(rename = "PGPORT", default = "default_port")]
    pub port: u16,
    #[serde(rename = "PGDATABASE")]
    pub database: String,
    #[serde(rename = "PGUSER")]
    pub username: String,
    #[serde(rename = "PGUSER")]
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub pg: PgConfig,
}

pub fn load() -> Result<Config, Error> {
    Ok(envy::from_env::<Config>()?)
}
