use crate::config::Config;
use crate::proto::errors::error_detail::Value;
use crate::proto::errors::{Error, ErrorDetail, ErrorResponse};

use base64::encode;
use bytes::Bytes;
use prost::Message;
use std::collections::HashMap;
use tonic::Status;

pub fn pg_url(config: &Config) -> String {
    format!(
        "postgres://{}:{}@{}/{}",
        config.pguser, config.pgpassword, config.pghost, config.pgdatabase
    )
}

pub fn with_details(status: Status, code: u32, message: &str, details: &[(&str, Value)]) -> Status {
    let mut d = HashMap::new();

    for (k, v) in details {
        d.insert(
            k.to_string(),
            ErrorDetail {
                value: Some(v.clone()),
            },
        );
    }

    let err = ErrorResponse {
        errors: vec![Error {
            code: code,
            message: message.to_owned(),
            details: d,
        }],
    };

    let mut buf = Vec::new();

    match err.encode(&mut buf) {
        Ok(_) => Status::with_details(status.code(), status.message(), Bytes::from(encode(buf))),
        Err(_) => Status::internal("Internal server error"),
    }
}
