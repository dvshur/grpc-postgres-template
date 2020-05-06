use crate::error::*;
use sqlx::PgPool;

#[derive(Debug)]
pub enum Platform {
    IOS,
    Android,
}

#[derive(Clone)]
pub struct Repo {
    pgpool: PgPool,
}

impl Repo {
    // pgpool should probably be borrowed in order to be reused between repos
    pub fn new(pgpool: &PgPool) -> Self {
        Repo {
            pgpool: pgpool.to_owned(),
        }
    }

    pub async fn subscribe(
        &self,
        topic: &str,
        device_id: &str,
        platform: Platform,
    ) -> Result<String, Error> {
        Ok(format!(
            "Subscribed to topic {}, device_id {}, platform {:?}",
            topic, device_id, platform
        ))
    }
}
