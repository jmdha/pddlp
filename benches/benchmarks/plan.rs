use criterion::{criterion_group, Criterion};
use pddlp::plan;

pub const PLAN: &'static str = r#"
(pickup b1)
(stack b1 b2)
; cost = 2 (unit cost)
"#;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-plan");
    group.throughput(criterion::Throughput::Bytes(PLAN.len() as u64));
    group.bench_function("parse-plan", |b| b.iter(|| plan::parse(PLAN)));
    group.finish();
}

criterion_group!(benches, bench);
