use crate::proto::errors::error_detail;
use crate::proto::push::notifier_server::Notifier;
use crate::proto::push::{Platform as PPlatform, SubscribeRequest, SubscribeResponse};
use crate::repo::*;
use crate::util::with_details;

use error_detail::Value;
use tonic::{Request, Response, Status};

pub struct NotifierImpl {
    repo: Repo,
}

impl NotifierImpl {
    pub fn new(repo: &Repo) -> Self {
        NotifierImpl {
            repo: repo.to_owned(),
        }
    }
}

#[tonic::async_trait]
impl Notifier for NotifierImpl {
    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeResponse>, Status> {
        println!("Subscribe request: {:?}", request);

        let req = request.into_inner();

        if req.platform() == PPlatform::Android {
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
            // todo handle Undefined case when implementing validation
            let platform = match req.platform() {
                PPlatform::Ios => Platform::IOS,
                _ => Platform::Android,
            };

            match self
                .repo
                .subscribe(&req.topic, &req.device_id, platform)
                .await
            {
                Ok(id) => {
                    let resp = SubscribeResponse {
                        subscription_id: id,
                    };
                    Ok(Response::new(resp))
                }
                Err(_) => Err(Status::internal("Internal server error")),
            }
        }
    }
}
