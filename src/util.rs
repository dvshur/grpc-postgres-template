use crate::config::PgConfig;

pub fn pg_url(config: &PgConfig) -> String {
    format!(
        "postgres://{}:{}@{}/{}",
        config.username, config.password, config.host, config.database
    )
}
