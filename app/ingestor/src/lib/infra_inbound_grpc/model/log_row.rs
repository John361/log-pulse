use std::collections::{BTreeMap, HashMap};

use prost_types::value::Kind;
use prost_types::{Struct, Timestamp, Value};

use crate::domain::model::LogRow;
use crate::grpc::log::LogEntryRequest;

impl From<LogRow> for LogEntryRequest {
    fn from(value: LogRow) -> Self {
        let timestamp = Some(Timestamp {
            seconds: value.timestamp / 1000,
            nanos: ((value.timestamp % 1000) * 1_000_000) as i32,
        });

        let metadata = if value.metadata.is_empty() || value.metadata == "{}" {
            None
        } else {
            let raw_map: HashMap<String, String> =
                serde_json::from_str(&value.metadata).unwrap_or_default();

            let mut fields = BTreeMap::new();
            for (k, v) in raw_map {
                fields.insert(
                    k,
                    Value {
                        kind: Some(Kind::StringValue(v)),
                    },
                );
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
