mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    benchmarks::domain::benches,
    benchmarks::problem::benches,
    benchmarks::plan::benches,
}
