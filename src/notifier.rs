use crate::proto::push::notifier_server::Notifier;
use crate::proto::push::{Platform, SubscribeRequest, SubscribeResponse};

use bytes::Bytes;
use tonic::{Code, Request, Response, Status};

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

        if request.into_inner().platform == Platform::Android.into() {
            Err(Status::with_details(
                Code::ResourceExhausted,
                "Android users are limited to 0 requests per second.",
                Bytes::new()
                // Bytes::from_static(b"hello"),
                // Bytes::from("sad"),
                // Bytes::from_static(&[1, 1, 1, 255]),
            ))
        } else {
            Ok(Response::new(resp))
        }
    }
}
