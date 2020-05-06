mod config;
mod error;
mod proto;
mod service;
mod util;

use proto::push::notifier_server::NotifierServer;
use service::NotifierImpl;
use tonic::transport::Server;
use util::pg_url;

use sqlx::postgres::PgPool;

// todo proper error handling with custom (or derived) error types
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
