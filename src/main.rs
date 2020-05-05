mod notifier;
mod proto;

use notifier::NotifierImpl;
use proto::push::notifier_server::NotifierServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:4000".parse()?;

    // its looks possible to customize headers using with_interceptor serer factory

    Server::builder()
        .add_service(NotifierServer::new(NotifierImpl::default()))
        .serve(addr)
        .await?;

    Ok(())
}
