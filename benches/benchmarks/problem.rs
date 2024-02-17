use criterion::{criterion_group, Criterion};
use pddlp::problem;

pub const PROBLEM: &'static str = r#"
;; base case
;;
(define (problem blocksworld-01)
 (:domain blocksworld)
 (:objects  b1 b2 - object)
 (:init 
    (arm-empty)
    (clear b2)
    (on-table b2)
    (clear b1)
    (on-table b1)
)
 (:goal (and 
    (clear b1)
    (on b1 b2)
    (on-table b2)
)))
"#;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-problem");
    group.throughput(criterion::Throughput::Bytes(PROBLEM.len() as u64));
    group.bench_function("parse-problem", |b| {
        b.iter(|| problem::parse(PROBLEM))
    });
    group.finish();
}

criterion_group!(benches, bench);
