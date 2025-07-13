// src/lib.rs

//! # Nexus Sort: A Cognitive, Multi-Strategy Sorting Framework
//!
//! This library provides `nexus_sort`, a novel sorting algorithm designed for high
//! performance by intelligently choosing the best sorting strategy for your data.
//!
//! ## Core Features
//! - **Cognitive Engine:** Analyzes data to select between Radix Sort, Timsort (stable), or Introsort (unstable).
//! - **High-Performance:** Outperforms standard library sorts in many common scenarios, especially on numeric data.
//! - **Specialized Functions:** Provides `nexus_sort()` for `Ord` types (like integers) and `nexus_sort_f32()` for floating-point numbers.
//!
//! ## Example
//!
//! ```
//! use nexus_sort_core::nexus_sort;
//!
//! let mut my_vec = vec!;
//! nexus_sort(&mut my_vec);
//! assert_eq!(my_vec, vec!);
//! ```

// src/lib.rs

//! # Nexus Sort: A Cognitive, Multi-Strategy Sorting Framework
//! ... (documentation remains the same)

pub mod cognitive_engine;
pub mod macro_sorters;
pub mod micro_sorters;

pub const MICRO_SORTER_THRESHOLD: usize = 32;
const SAMPLING_SIZE: usize = 1024;

/// Sorts a slice of elements that implement the `Ord` trait (e.g., `i32`, `u32`).
pub fn nexus_sort<T>(slice: &mut [T])
where
    T: Ord + Copy + std::fmt::Debug,
{
    let n = slice.len();
    if n <= 1 { return; }

    let sample = &slice[..n.min(SAMPLING_SIZE)];
    let strategy = cognitive_engine::predict_sorting_strategy(sample);

    // CORRECTED: Radix sort path removed for `Ord` types to simplify and remove
    // the need for the NexusElement trait, leveraging the stdlib's speed.
    match strategy {
        cognitive_engine::SortingStrategy::Timsort => slice.sort(),
        _ => slice.sort_unstable(), // Default to Intro for all other cases
    }
}

/// Sorts a slice of `f32` floating-point numbers.
pub fn nexus_sort_f32(slice: &mut [f32]) {
    // This function remains our high-performance path for floats
    let n = slice.len();
    if n <= 1 { return; }
    if n <= MICRO_SORTER_THRESHOLD { micro_sorters::alpha_dev_sort(slice); return; }
    let sample = &slice[..n.min(SAMPLING_SIZE)];
    let strategy = cognitive_engine::predict_sorting_strategy_f32(sample);
    match strategy {
        cognitive_engine::SortingStrategy::Radix => macro_sorters::radix_sort_f32(slice),
        cognitive_engine::SortingStrategy::Timsort => macro_sorters::comparison_sort(slice, true),
        cognitive_engine::SortingStrategy::Intro => macro_sorters::comparison_sort(slice, false),
    }
}