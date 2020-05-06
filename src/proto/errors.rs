#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    #[prost(message, repeated, tag = "1")]
    pub errors: ::std::vec::Vec<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
    #[prost(map = "string, message", tag = "3")]
    pub details: ::std::collections::HashMap<std::string::String, ErrorDetail>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetail {
    #[prost(oneof = "error_detail::Value", tags = "1, 2, 3, 4")]
    pub value: ::std::option::Option<error_detail::Value>,
}
pub mod error_detail {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(std::string::String),
        #[prost(double, tag = "2")]
        Double(f64),
        #[prost(int64, tag = "3")]
        Long(i64),
        #[prost(bytes, tag = "4")]
        Binary(std::vec::Vec<u8>),
    }
}
