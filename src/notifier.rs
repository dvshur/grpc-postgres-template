use crate::proto::push::notifier_server::Notifier;
use crate::proto::push::{SubscribeRequest, SubscribeResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NotifierImpl {}

// todo set headers/trailers on both failed and succesful requests

#[tonic::async_trait]
impl Notifier for NotifierImpl {
    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeResponse>, Status> {
        println!("Subscribe request: {:?}", request);

        let resp = SubscribeResponse {
            subscription_id: String::from("blah blah"),
        };

        Ok(Response::new(resp))
    }
}
