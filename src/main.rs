mod config;
mod error;
mod proto;
mod service;
mod util;

use error::*;
use proto::push::notifier_server::NotifierServer;
use service::NotifierImpl;
use sqlx::postgres::PgPool;
use tonic::transport::Server;
use util::pg_url;

// todo repo
// todo validation

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = crate::config::load()?;

    let addr = format!("0.0.0.0:{}", config.port).parse()?;

    let _pool = PgPool::builder()
        .max_size(config.pg.pool_size)
        .build(&pg_url(&config.pg))
        .await?;

    // It is possible to customize error details header using lower level workaround.
    // See hyper_warp example, as suggested in discord.
    Server::builder()
        .add_service(NotifierServer::new(NotifierImpl::default()))
        .serve(addr)
        .await?;

    Ok(())
}
