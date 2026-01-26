use serde_json::Value;

pub struct LogRow { // Warning: copy / paste from ingestor
    pub timestamp: i64,
    pub level: i8,
    pub service_name: String,
    pub message: String,
    pub metadata: String,
}

pub struct LogFeature {
    level_val: f64,
    msg_length: f64,
    metadata_count: f64,
    // TODO: add day hour to detect temporal anomalies
}

impl LogFeature {
    pub fn from_log(log: &LogRow) -> Vec<f64> {
        vec![
            (log.level as f64 / 4.0) * 3.0,
            (log.message.len() as f64).min(1000.0) / 1000.0,
            (serde_json::from_str::<Value>(&log.metadata)
                .map(|v| v.as_object().map(|m| m.len()).unwrap_or(0))
                .unwrap_or(0) as f64).min(20.0) / 20.0,
        ]
    }
}
