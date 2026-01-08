use std::collections::BTreeMap;

use prost_types::{Struct, Timestamp, Value};
use prost_types::value::Kind;

use crate::domain::model::LogRow;
use crate::grpc::log::LogEntryRequest;

impl From<LogRow> for LogEntryRequest {
    fn from(value: LogRow) -> Self {
        let timestamp = Some(Timestamp {
            seconds: value.timestamp / 1000,
            nanos: ((value.timestamp % 1000) * 1_000_000) as i32,
        });

        let metadata = if value.metadata.is_empty() {
            None
        } else {
            let mut fields = BTreeMap::new();
            for (k, v) in value.metadata {
                fields.insert(k, Value {
                    kind: Some(Kind::StringValue(v)),
                });
            }
            Some(Struct { fields })
        };

        LogEntryRequest {
            timestamp,
            level: value.level as i32,
            service_name: value.service_name,
            message: value.message,
            metadata,
        }
    }
}
