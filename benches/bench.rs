use std::time::Duration;
use criterion::Throughput;
use betterquesting_report::parsers::load_data;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Load data");
    group.throughput(Throughput::Bytes(11229479)); // sample/1/ size
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("load_data all", |b| b.iter(|| load_data(black_box("./sample/1"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);