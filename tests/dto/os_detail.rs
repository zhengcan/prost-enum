#[derive(Clone, PartialEq, ::prost::Oneof)]
#[prost_enum::enhance]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum Detail {
    #[prost(message, tag = "2")]
    Ios(Ios),
    #[prost(message, tag = "3")]
    Android(Android),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ios {
    #[prost(string, tag = "1")]
    pub build_number: ::prost::alloc::string::String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Android {
    #[prost(string, tag = "1")]
    pub version_code: ::prost::alloc::string::String,
}
