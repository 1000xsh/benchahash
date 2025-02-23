use ahash::AHasher;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dashmap::DashMap;
use std::hash::BuildHasherDefault;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

type AHashDashMap<K, V> = DashMap<K, V, BuildHasherDefault<AHasher>>;

const BENCHMARK_DURATION: u64 = 10; // seconds
const NUM_ELEMENTS: u64 = 100_000;
const NUM_THREADS: usize = 4;
const SAMPLE_SIZE: usize = 50;

fn benchmark_dashmap_siphash(c: &mut Criterion) {
    let mut group = c.benchmark_group("dashMap with siphash");
    group.measurement_time(std::time::Duration::new(BENCHMARK_DURATION, 0));
    group.sample_size(SAMPLE_SIZE);
    group.throughput(Throughput::Elements(NUM_ELEMENTS));

    group.bench_function(BenchmarkId::new("single-threaded", NUM_ELEMENTS), |b| {
        b.iter(|| {
            let start = Instant::now();
            let map: DashMap<u64, u64> = DashMap::new();
            for i in 0..NUM_ELEMENTS {
                map.insert(black_box(i), black_box(i));
            }
            println!("siphash single-threaded duration: {:?}", start.elapsed());
        })
    });

    group.finish();
}

fn benchmark_dashmap_ahash(c: &mut Criterion) {
    let mut group = c.benchmark_group("dashmap with ahash");
    group.measurement_time(std::time::Duration::new(BENCHMARK_DURATION, 0));
    group.sample_size(SAMPLE_SIZE);
    group.throughput(Throughput::Elements(NUM_ELEMENTS));

    group.bench_function(BenchmarkId::new("single-threaded", NUM_ELEMENTS), |b| {
        b.iter(|| {
            let start = Instant::now();
            let map: AHashDashMap<u64, u64> = DashMap::with_hasher(BuildHasherDefault::default());
            for i in 0..NUM_ELEMENTS {
                map.insert(black_box(i), black_box(i));
            }
            println!("ahash single-threaded duration: {:?}", start.elapsed());
        })
    });

    group.finish();
}

fn benchmark_dashmap_siphash_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent dashmap with siphash");
    group.measurement_time(std::time::Duration::new(BENCHMARK_DURATION, 0));
    group.sample_size(SAMPLE_SIZE);
    group.throughput(Throughput::Elements(NUM_ELEMENTS));

    group.bench_function(BenchmarkId::new("concurrent", NUM_ELEMENTS), |b| {
        b.iter(|| {
            let start = Instant::now();
            let map: Arc<DashMap<u64, u64>> = Arc::new(DashMap::new());
            let mut handles = vec![];

            for _ in 0..NUM_THREADS {
                let map = Arc::clone(&map);
                handles.push(thread::spawn(move || {
                    for i in 0..NUM_ELEMENTS / NUM_THREADS as u64 {
                        map.insert(black_box(i), black_box(i));
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
            println!("siphash concurrent duration: {:?}", start.elapsed());
        })
    });

    group.finish();
}

fn benchmark_dashmap_ahash_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent dashmap with ahash");
    group.measurement_time(std::time::Duration::new(BENCHMARK_DURATION, 0));
    group.sample_size(SAMPLE_SIZE);
    group.throughput(Throughput::Elements(NUM_ELEMENTS));

    group.bench_function(BenchmarkId::new("concurrent", NUM_ELEMENTS), |b| {
        b.iter(|| {
            let start = Instant::now();
            let map: Arc<AHashDashMap<u64, u64>> =
                Arc::new(DashMap::with_hasher(BuildHasherDefault::default()));
            let mut handles = vec![];

            for _ in 0..NUM_THREADS {
                let map = Arc::clone(&map);
                handles.push(thread::spawn(move || {
                    for i in 0..NUM_ELEMENTS / NUM_THREADS as u64 {
                        map.insert(black_box(i), black_box(i));
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
            println!("ahash concurrent duration: {:?}", start.elapsed());
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_dashmap_siphash,
    benchmark_dashmap_ahash,
    benchmark_dashmap_siphash_concurrent,
    benchmark_dashmap_ahash_concurrent
);
criterion_main!(benches);
