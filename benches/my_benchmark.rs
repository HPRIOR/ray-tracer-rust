use criterion::{criterion_group, criterion_main, Criterion};
use module_lib::exercises::world_ex::world_ex::render_world;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("render world", |b| b.iter(|| render_world(75)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
