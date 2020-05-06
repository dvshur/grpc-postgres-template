mod config;
mod notifier;
mod proto;
mod status;
mod util;

use notifier::NotifierImpl;
use proto::push::notifier_server::NotifierServer;
use tonic::transport::Server;
use util::pg_url;

use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = crate::config::load()?;

    let addr = format!("0.0.0.0:{}", config.port).parse()?;

    let pool = PgPool::builder()
        .max_size(config.pg.pool_size)
        .build(&pg_url(&config.pg))
        .await?;

    // it is possible to customize headers using lower level workaround
    // see hyper_warp example, as suggested in discord
    Server::builder()
        .add_service(NotifierServer::new(NotifierImpl::default()))
        .serve(addr)
        .await?;

    Ok(())
}
