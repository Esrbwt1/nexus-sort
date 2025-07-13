// src/macro_sorters.rs
use crate::micro_sorters::alpha_dev_sort;
use crate::MICRO_SORTER_THRESHOLD;
use std::cmp::Ordering;

// This function is now specifically for f32
pub fn radix_sort_f32(array: &mut [f32]) {
    if array.len() <= 1 { return; }
    const SIGN_MASK: u32 = 1 << 31;

    // We can directly cast the f32 slice to a u32 slice to work with its bits
    let unsigned_slice = unsafe { std::slice::from_raw_parts_mut(array.as_mut_ptr() as *mut u32, array.len()) };
    
    // 1. Transform f32 to order-preserving u32
    for item in unsigned_slice.iter_mut() {
        if *item & SIGN_MASK != 0 { // Negative number
            *item = !*item; // Invert all bits
        } else { // Positive number
            *item ^= SIGN_MASK; // Flip sign bit
        }
    }

    // --- The u32 Radix Sort Logic ---
    // This part is now embedded here
    let mut buffer = vec![0u32; unsigned_slice.len()];
    for pass in 0..4 {
        let shift = pass * 8;
        let mut counts = [0usize; 256];
        for &item in unsigned_slice.iter() {
            let byte = ((item >> shift) & 0xFF) as usize;
            counts[byte] += 1;
        }
        for i in 1..256 {
            counts[i] += counts[i - 1];
        }
        for &item in unsigned_slice.iter().rev() {
            let byte = ((item >> shift) & 0xFF) as usize;
            counts[byte] -= 1;
            buffer[counts[byte]] = item;
        }
        unsigned_slice.copy_from_slice(&buffer);
    }
    // --- End of u32 Radix Sort Logic ---

    // 3. Transform back to f32
    for item in unsigned_slice.iter_mut() {
        if *item & SIGN_MASK != 0 { // Originally positive
            *item ^= SIGN_MASK;
        } else { // Originally negative
            *item = !*item;
        }
    }
}


pub fn comparison_sort(array: &mut [f32], prefer_stable: bool) {
    if prefer_stable {
        array.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    } else {
        introsort(array);
    }
}

fn introsort(array: &mut [f32]) {
    if array.len() <= MICRO_SORTER_THRESHOLD {
        alpha_dev_sort(array);
        return;
    }
    introsort_recursive(array, (array.len() as f64).log2() as i32 * 2);
}

fn introsort_recursive(array: &mut [f32], limit: i32) {
    let n = array.len();
    if n <= MICRO_SORTER_THRESHOLD { alpha_dev_sort(array); return; }
    if limit == 0 { heapsort(array); return; }
    if let Some(p) = partition(array) {
        let (left, right) = array.split_at_mut(p);
        introsort_recursive(left, limit - 1);
        introsort_recursive(&mut right[1..], limit - 1);
    } else {
        array.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    }
}

fn partition(array: &mut [f32]) -> Option<usize> {
    let n = array.len(); let mid = n / 2; let last = n - 1;
    if array[0].partial_cmp(&array[mid]).is_none() { return None; }
    if array[0].partial_cmp(&array[mid]) == Some(Ordering::Greater) { array.swap(0, mid); }
    if array[0].partial_cmp(&array[last]) == Some(Ordering::Greater) { array.swap(0, last); }
    if array[mid].partial_cmp(&array[last]) == Some(Ordering::Greater) { array.swap(mid, last); }
    array.swap(mid, last);
    let mut i = 0;
    for j in 0..last {
        if array[j].partial_cmp(&array[last]) != Some(Ordering::Greater) {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, last);
    Some(i)
}

fn heapsort(array: &mut [f32]) {
    let n = array.len(); if n <= 1 { return; }
    for i in (0..n / 2).rev() { heapify(array, n, i); }
    for i in (0..n).rev() { array.swap(0, i); heapify(array, i, 0); }
}

fn heapify(array: &mut [f32], n: usize, i: usize) {
    let mut largest = i; let l = 2 * i + 1; let r = 2 * i + 2;
    if l < n && array[l].partial_cmp(&array[largest]) == Some(Ordering::Greater) { largest = l; }
    if r < n && array[r].partial_cmp(&array[largest]) == Some(Ordering::Greater) { largest = r; }
    if largest != i { array.swap(i, largest); heapify(array, n, largest); }
}