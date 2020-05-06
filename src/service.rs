use crate::proto::errors::error_detail;
use crate::proto::push::notifier_server::Notifier;
use crate::proto::push::{Platform, SubscribeRequest, SubscribeResponse};
use crate::util::with_details;

use error_detail::Value;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NotifierImpl {}

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
            Err(with_details(
                Status::resource_exhausted("Android clients are limited to 0 requests per second"),
                950000,
                "Not a good day, mate",
                &[(
                    "reason",
                    Value::String(String::from("It's april fool's day")),
                )],
            ))
        } else {
            Ok(Response::new(resp))
        }
    }
}
