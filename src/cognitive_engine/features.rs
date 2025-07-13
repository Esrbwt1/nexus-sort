// src/cognitive_engine/features.rs
use std::cmp::Ordering;

pub struct DataFeatures {
    pub pre_sortedness_score: f64,
    pub is_numeric_like: bool,
}

pub fn extract_features_ord<T: Ord>(data_sample: &[T]) -> DataFeatures {
    let n = data_sample.len();
    if n <= 1 { return DataFeatures { pre_sortedness_score: 1.0, is_numeric_like: true }; }
    let mut runs = 1;
    for i in 1..n { if data_sample[i] < data_sample[i - 1] { runs += 1; } }
    let score = 1.0 - ((runs - 1) as f64 / (n - 1) as f64);
    DataFeatures { pre_sortedness_score: score, is_numeric_like: true }
}

pub fn extract_features_f32(data_sample: &[f32]) -> DataFeatures {
    let n = data_sample.len();
    if n <= 1 { return DataFeatures { pre_sortedness_score: 1.0, is_numeric_like: true }; }
    let mut runs = 1;
    for i in 1..n {
        if data_sample[i].partial_cmp(&data_sample[i - 1]) == Some(Ordering::Less) { runs += 1; }
    }
    let score = 1.0 - ((runs - 1) as f64 / (n - 1) as f64);
    let numeric = !data_sample.iter().any(|v| v.is_nan());
    DataFeatures { pre_sortedness_score: score, is_numeric_like: numeric }
}