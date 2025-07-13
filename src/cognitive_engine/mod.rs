// src/cognitive_engine/mod.rs
pub mod features;

#[derive(Debug, PartialEq)]
pub enum SortingStrategy { Radix, Timsort, Intro }

pub fn predict_sorting_strategy<T: Ord>(data_sample: &[T]) -> SortingStrategy {
    let features = features::extract_features_ord(data_sample);
    if features.pre_sortedness_score > 0.8 {
        return SortingStrategy::Timsort;
    }
    // Default to Intro for integers now
    SortingStrategy::Intro
}

pub fn predict_sorting_strategy_f32(data_sample: &[f32]) -> SortingStrategy {
    let features = features::extract_features_f32(data_sample);
    if features.pre_sortedness_score > 0.8 { return SortingStrategy::Timsort; }
    if features.is_numeric_like { return SortingStrategy::Radix; }
    SortingStrategy::Intro
}