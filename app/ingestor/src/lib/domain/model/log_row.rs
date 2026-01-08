use std::collections::HashMap;

use clickhouse::Row;
use serde::{Deserialize, Serialize};

use crate::grpc::log::LogEntryRequest;

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct LogRow {
    pub timestamp: i64,
    pub level: i8,
    pub service_name: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

impl From<LogEntryRequest> for LogRow {
    fn from(value: LogEntryRequest) -> Self {
        let ts = value.timestamp.unwrap_or_default();
        let millis = (ts.seconds * 1000) + (ts.nanos as i64 / 1_000_000);
        let mut metadata_map = HashMap::new();

        if let Some(struct_data) = value.metadata {
            for (k, v) in struct_data.fields {
                if let Some(kind) = v.kind {
                    use prost_types::value::Kind;

                    let val_str = match kind {
                        Kind::StringValue(s) => s,
                        Kind::NumberValue(n) => n.to_string(),
                        Kind::BoolValue(b) => b.to_string(),
                        Kind::NullValue(_) => "null".to_string(),
                        _ => "complex_object".to_string(),
                    };

                    metadata_map.insert(k, val_str);
                }
            }
        }

        LogRow {
            timestamp: millis,
            level: value.level as i8,
            service_name: value.service_name,
            message: value.message,
            metadata: HashMap::new(),
        }
    }
}

impl From<LogRow> for LogEntryRequest { // TODO: move in grpc module
    fn from(value: LogRow) -> Self {
        LogEntryRequest {
            timestamp: None,
            level: value.level as i32,
            service_name: value.service_name,
            message: value.message,
            metadata: None,
        }
    }
}
