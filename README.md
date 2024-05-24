# pddlp

[![Rust](https://github.com/jamadaha/seeker/actions/workflows/rust.yml/badge.svg)](https://github.com/jamadaha/seeker/actions/workflows/rust.yml)

![GitHub last commit (branch)](https://img.shields.io/github/last-commit/jamadaha/seeker/master)
![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/m/jamadaha/seeker)

![Crates.io Version](https://img.shields.io/crates/v/pddlp)
![Crates.io Total Downloads](https://img.shields.io/crates/d/pddlp)



_A simple, fast, and robust PDDL parser_

**pddlp** has three goals:

+ *Robust* - Is tested on commenly used PDDL domains and problems
+ *Fast* - Optimised and benchmarked to be as fast as possible
+ *Simple* - Avoids rarely used PDDL syntax in favor of easy of use

## Example
```rust
let input = "(define (problem prob)
                  (:objects o1)
                  (:init (p o1))
                  (:goal (not (p o3)))
             )";
let problem = pddlp::problem::parse(&input)?;
assert_eq!(problem.name, Some("prob"));
assert_eq!(problem.domain, None);
assert_eq!(problem.objects, Some(vec![pddlp::problem::Object { name: "o1", type_name: None }]));
//...
```

## Benchmark
Benchmarked on a AMD Ryzen 5 5600X 6-Core Processor × 6 with [Criterion](https://github.com/bheisler/criterion.rs)

|               | Throughput | Time to Parse (see [here](https://github.com/jamadaha/pddlp/tree/master/benches/benchmarks)) |
|---------------|------------|--------|
| Domain        | 436 MiB/s  |   2 µs |
| Problem       | 468 MiB/s  | 523 ns |
| Plan          | 507 MiB/s  |  93 ns |
