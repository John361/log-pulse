use std::collections::{BTreeMap, HashMap};

use clickhouse::Row;
use prost_types::value::Kind;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::grpc::log::LogEntryRequest;

#[derive(Clone, Debug, Row, Serialize, Deserialize)]
pub struct LogRow {
    pub timestamp: i64,
    pub level: i8,
    pub service_name: String,
    pub message: String,
    pub metadata: String,
}

impl From<LogEntryRequest> for LogRow {
    fn from(value: LogEntryRequest) -> Self {
        let ts = value.timestamp.unwrap_or_default();
        let millis = (ts.seconds * 1000) + (ts.nanos as i64 / 1_000_000);
        let mut metadata_map = HashMap::new();

        if let Some(struct_data) = value.metadata {
            for (k, v) in struct_data.fields {
                if let Some(kind) = v.kind {
                    let val_str = stringify_kind(kind);
                    metadata_map.insert(k, val_str);
                }
            }
        }

        LogRow {
            timestamp: millis,
            level: value.level as i8,
            service_name: value.service_name,
            message: value.message,
            metadata: serde_json::to_string(&metadata_map).unwrap_or_default(),
        }
    }
}

fn stringify_kind(kind: Kind) -> String {
    match kind {
        Kind::StringValue(s) => s,
        Kind::NumberValue(n) => n.to_string(),
        Kind::BoolValue(b) => b.to_string(),
        Kind::NullValue(_) => "null".to_string(),
        Kind::StructValue(s) => {
            let json_val = prost_struct_to_json(s.fields);
            serde_json::to_string(&json_val).unwrap_or_else(|_| "{}".to_string())
        }
        Kind::ListValue(l) => {
            let json_list: Vec<Value> = l.values
                .into_iter()
                .filter_map(|v| v.kind.map(prost_value_to_json))
                .collect();
            serde_json::to_string(&json_list).unwrap_or_else(|_| "[]".to_string())
        }
    }
}

fn prost_struct_to_json(fields: BTreeMap<String, prost_types::Value>) -> Value {
    let mut map = serde_json::Map::new();

    for (key, value) in fields {
        if let Some(kind) = value.kind {
            map.insert(key, prost_value_to_json(kind));
        }
    }

    Value::Object(map)
}

fn prost_value_to_json(kind: Kind) -> Value {
    use prost_types::value::Kind;
    match kind {
        Kind::NullValue(_) => Value::Null,
        Kind::NumberValue(n) => serde_json::Number::from_f64(n)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        Kind::StringValue(s) => Value::String(s),
        Kind::BoolValue(b) => Value::Bool(b),
        Kind::StructValue(s) => prost_struct_to_json(s.fields),
        Kind::ListValue(l) => {
            Value::Array(l.values.into_iter()
                .filter_map(|v| v.kind.map(prost_value_to_json))
                .collect())
        }
    }
}
