mod config;
mod error;
mod proto;
mod repo;
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

    let pgpool = PgPool::builder()
        .max_size(config.pgpool)
        .build(&pg_url(&config))
        .await?;

    let repo = repo::Repo::new(&pgpool);

    // It is possible to customize error details header using lower level workaround.
    // See hyper_warp example, as suggested in discord.
    Server::builder()
        .add_service(NotifierServer::new(NotifierImpl::new(&repo)))
        .serve(addr)
        .await?;

    Ok(())
}
