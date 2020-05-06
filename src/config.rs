use crate::error::*;
use serde::Deserialize;

fn default_port() -> u16 {
    4000
}

fn default_pgport() -> u16 {
    5432
}

fn default_pgpool() -> u32 {
    4
}

// nested non-string values are not supported due to issue
// https://github.com/softprops/envy/issues/26
// so using a simple flat config for now

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_pgpool")]
    pub pgpool: u32,
    pub pghost: String,
    #[serde(default = "default_pgport")]
    pub pgport: u16,
    pub pgdatabase: String,
    pub pguser: String,
    pub pgpassword: String,
}

pub fn load() -> Result<Config, Error> {
    Ok(envy::from_env::<Config>()?)
}
