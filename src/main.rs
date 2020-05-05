mod proto;

use tonic::{transport::Server, Request, Response, Status};

use proto::push::notifier_server::{Notifier, NotifierServer};
// use proto::push::{SubscribeRequest, SubscribeResponse};

fn main() {
    println!("Hello, world!");
}
