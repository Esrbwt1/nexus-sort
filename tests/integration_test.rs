// tests/integration_test.rs
use nexus_sort_core::{nexus_sort, nexus_sort_f32};
use rand::{Rng, SeedableRng};

#[test]
fn test_i32_sort() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut data: Vec<i32> = (0..1000).map(|_| rng.r#gen()).collect();
    let mut sorted = data.clone();
    sorted.sort();
    nexus_sort(&mut data);
    assert_eq!(data, sorted);
}

#[test]
fn test_f32_sort() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(44);
    let mut data: Vec<f32> = (0..1000).map(|_| rng.r#gen::<f32>() * 2000.0 - 1000.0).collect();
    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nexus_sort_f32(&mut data);
    assert_eq!(data, sorted);
}