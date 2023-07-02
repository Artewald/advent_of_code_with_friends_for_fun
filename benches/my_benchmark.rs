use criterion::{criterion_group, criterion_main, Criterion};
use cool::file_reader::parse_file;
use cool::tim_part2::parse_file_tim;
use cool::alwaa_2::parse_best;
use cool::calorie_counter::sum_calories;

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("parse_file", |b| b.iter(|| parse_file("./text.txt")));
    let mut group = c.benchmark_group("parse_file");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(10));
    group.bench_function("alex", |b| b.iter(|| parse_best("./text.txt")));
    group.bench_function("tim", |b| b.iter(|| parse_file_tim("./text.txt")));
    group.bench_function("sverre", |b| b.iter(|| sum_calories("./text.txt", 3)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);