use std::collections::HashMap;
use std::time::{SystemTime};

use crate::grpc_wrapper::raw_topic_service::common::codecs::RawSupportedCodecs;
use ydb_grpc::google_proto_workaround::protobuf::Timestamp;
use ydb_grpc::ydb_proto::topic::{Consumer, SupportedCodecs};

#[derive(serde::Serialize, Clone)]
pub(crate) struct RawConsumer {
    pub name: String,
    pub important: bool,
    pub read_from: SystemTime,
    pub supported_codecs: RawSupportedCodecs,
    pub attributes: HashMap<String, String>,
}

impl From<RawConsumer> for Consumer {
    fn from(value: RawConsumer) -> Self {
        Self {
            name: value.name,
            important: value.important,
            read_from: Some(Timestamp {
                seconds: value.read_from.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                nanos: value.read_from.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i32,
            }),
            supported_codecs: Some(SupportedCodecs::from(value.supported_codecs)),
            attributes: value.attributes,
        }
    }
}
