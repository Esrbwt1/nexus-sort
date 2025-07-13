// src/micro_sorters.rs
use std::cmp::Ordering;

/// The main entry point for our micro-sorter.
/// It dispatches to the optimal sorting network based on size.
pub fn alpha_dev_sort<T: PartialOrd>(subarray: &mut [T]) {
    match subarray.len() {
        0..=1 => { /* Already sorted */ }
        2 => { sort2(subarray); }
        3 => { sort3(subarray); }
        4 => { sort4_alphadev(subarray); }
        // For other small sizes, we fall back to a safe sort.
        // We can add more networks later for 5, 6, 7, 8...
        _ => { fallback_sort(subarray) }
    }
}

/// A safe fallback sort for sizes where we don't have an optimal network.
fn fallback_sort<T: PartialOrd>(subarray: &mut [T]) {
    subarray.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
}

// --- Sorting Networks ---

#[inline(always)]
fn swap_if_greater<T: PartialOrd>(arr: &mut [T], i: usize, j: usize) {
    if arr[i].partial_cmp(&arr[j]) == Some(Ordering::Greater) {
        arr.swap(i, j);
    }
}

fn sort2<T: PartialOrd>(arr: &mut [T]) {
    swap_if_greater(arr, 0, 1);
}

fn sort3<T: PartialOrd>(arr: &mut [T]) {
    swap_if_greater(arr, 0, 1);
    swap_if_greater(arr, 1, 2);
    swap_if_greater(arr, 0, 1);
}

/// The sorting network for 4 elements discovered by AlphaDev.
/// It uses the same number of swaps but can be more efficient on some CPUs.
fn sort4_alphadev<T: PartialOrd>(arr: &mut [T]) {
    swap_if_greater(arr, 0, 2);
    swap_if_greater(arr, 1, 3);
    swap_if_greater(arr, 0, 1);
    swap_if_greater(arr, 2, 3);
    swap_if_greater(arr, 1, 2);
}