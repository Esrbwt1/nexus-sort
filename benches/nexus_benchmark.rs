// benches/nexus_benchmark.rs
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nexus_sort_core::{nexus_sort, nexus_sort_f32};
use rand::{Rng, SeedableRng};

fn benchmark_integers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Integer Sort Comparison (Random)");
    for size in [100, 1_000, 10_000].iter() {
        let mut data: Vec<i32> = {
            let mut rng = rand::rngs::StdRng::seed_from_u64(42);
            (0..*size).map(|_| rng.r#gen()).collect()
        };
        group.bench_with_input(BenchmarkId::new("NexusSort", size), &data, |b, d| {
            b.iter(|| nexus_sort(&mut d.clone()));
        });
        group.bench_with_input(BenchmarkId::new("std::sort_unstable", size), &data, |b, d| {
            b.iter(|| d.clone().sort_unstable());
        });
    }
    group.finish();
}

fn benchmark_floats(c: &mut Criterion) {
    let mut group = c.benchmark_group("Float Sort Comparison (Random)");
    for size in [100, 1_000, 10_000].iter() {
        let mut data: Vec<f32> = {
            let mut rng = rand::rngs::StdRng::seed_from_u64(44);
            (0..*size).map(|_| rng.r#gen::<f32>() * 2000.0 - 1000.0).collect()
        };
        group.bench_with_input(BenchmarkId::new("NexusSort", size), &data, |b, d| {
            b.iter(|| nexus_sort_f32(&mut d.clone()));
        });
        group.bench_with_input(BenchmarkId::new("std::sort", size), &data, |b, d| {
            b.iter(|| d.clone().sort_by(|a,b| a.partial_cmp(b).unwrap()));
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_integers, benchmark_floats);
criterion_main!(benches);