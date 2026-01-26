use linfa::{Dataset, ParamGuard};
use linfa::prelude::{Fit, Predict};
use linfa_clustering::KMeans;
use ndarray::{Array1, Array2};

use crate::domain::model::{LogFeature, LogRow};

pub struct DetectorBusiness {

}

impl DetectorBusiness {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn detect_anomalies(logs: Vec<LogRow>) {
        if logs.len() < 10 {
            tracing::debug!("Not enough logs for anomaly detection: {}", logs.len());
            return;
        }

        let mut features: Vec<f64> = Vec::with_capacity(logs.len() * 3);

        for log in &logs {
            let feature = LogFeature::from_log(log);
            features.push(feature[0]);
            features.push(feature[1]);
            features.push(feature[2]);
        }

        let records: Array2<f64> = Array2::from_shape_vec((logs.len(), 3), features)
            .expect("Failed to create dataset array");
        let dataset = Dataset::new(records, Array1::<()>::default(logs.len()));
        let n_clusters = std::cmp::min(5, (logs.len() / 3).max(2));

        tracing::debug!(
            "Running KMeans with {} samples, {} clusters",
            logs.len(),
            n_clusters
        );

        let model = KMeans::params(n_clusters)
            .max_n_iterations(200)
            .tolerance(1e-3)
            .check()
            .expect("Invalid KMeans parameters")
            .fit(&dataset)
            .expect("KMeans fitting failed");

        let predictions = model.predict(&dataset);
        let centroids = model.centroids();
        let mut cluster_distances: Vec<Vec<f64>> = vec![Vec::new(); n_clusters];

        for (idx, &cluster) in predictions.iter().enumerate() {
            let sample = dataset.records().row(idx);
            let centroid = centroids.row(cluster);

            let distance = sample.iter()
                .zip(centroid.iter())
                .map(|(s, c)| (s - c).powi(2))
                .sum::<f64>()
                .sqrt();

            cluster_distances[cluster].push(distance);
        }

        let mut thresholds: Vec<f64> = Vec::with_capacity(n_clusters);

        for distances in &cluster_distances {
            if distances.is_empty() {
                thresholds.push(0.0);
                continue;
            }

            let mean: f64 = distances.iter().sum::<f64>() / distances.len() as f64;
            let variance: f64 = distances.iter()
                .map(|&d| (d - mean).powi(2))
                .sum::<f64>() / distances.len() as f64;
            let std_dev = variance.sqrt();

            thresholds.push(mean + 3.0 * std_dev);
        }

        let mut anomaly_count = 0;

        for (idx, &cluster) in predictions.iter().enumerate() {
            let sample = dataset.records().row(idx);
            let centroid = centroids.row(cluster as usize);

            let distance = sample.iter()
                .zip(centroid.iter())
                .map(|(s, c)| (s - c).powi(2))
                .sum::<f64>()
                .sqrt();

            let threshold = thresholds[cluster as usize];

            if distance > threshold && threshold > 0.0 {
                anomaly_count += 1;
                let anomaly = &logs[idx];
                tracing::warn!(
                "Anomaly detected! Cluster: {}, Distance: {:.3}, Threshold: {:.3}, Service: {}, Level: {}, Message: {}",
                cluster,
                distance,
                threshold,
                anomaly.service_name,
                anomaly.level,
                anomaly.message
            );
            }
        }

        tracing::debug!("Cluster sizes: {:?}",
            cluster_distances.iter().map(|d| d.len()).collect::<Vec<_>>()
        );
        tracing::debug!("Thresholds: {:?}",
            thresholds.iter().map(|&t| format!("{:.3}", t)).collect::<Vec<_>>()
        );

        tracing::info!(
            "Anomaly detection complete. Found {} anomalies out of {} logs ({}%)",
            anomaly_count,
            logs.len(),
            (anomaly_count as f64 / logs.len() as f64 * 100.0).round()
        );
    }
}
