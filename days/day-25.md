# Day 25 — Measure before optimizing

**Time box:** 60 minutes.

## Learning Objectives
- Run a repeatable measurement and distinguish benchmark from profiling.
- Explain release-mode effects and a performance trade-off.

## Why This Matters
Rust can be fast, but assumptions about allocation, cloning, and async overhead are still assumptions.

## Concepts
A benchmark measures a defined workload; a profiler attributes time or memory. Release builds enable optimization. Stack values are usually cheap, while owned `String`/`Vec` buffers allocate on the heap; moves transfer ownership, clones can allocate. Iterators and generics are commonly zero-cost after optimization, but async/task/lock overhead is measurable. Compare equivalent work, warm up, repeat, and avoid conclusions from one noisy run. Do not benchmark synthetic micro-work as a proxy for service latency. CPU and memory profilers answer different questions; use a platform profiler (such as Instruments on macOS) or an appropriate Rust allocation profiler after identifying a real need.

## Explanation
The standard library has no stable built-in benchmark harness. Use `std::hint::black_box` in a small repeatable timing loop for exploration, or add Criterion as an optional deliberate dependency when learning statistical benchmarks.

## Main Project
Choose name validation or code counting, define realistic input sizes, and record a baseline before changing implementation.

## Coding Exercises
1. Compare repeated `String::clone()` with borrowing in a release build; keep result observable and explain what the measurement does not prove.

## Quiz
1. What question does a profiler answer that a benchmark alone may not?
2. Why compare optimized release builds?

## Test Your Understanding
Identify a representative request-level metric and a possible microbenchmark trap.

## Reference Solutions
See instructor exercise solution Day 25.

## Common Mistakes
Optimizing before measuring; comparing different work; treating noisy wall-clock timing as proof.

## Production Perspective
Measure latency distributions, throughput, allocation/memory, and contention under realistic load.

## Architecture Perspective
Performance changes can increase complexity; require evidence and retain correctness tests.

## Project Change
### What We Are Adding
A repeatable measurement exercise; no optimization claim.
### Why We Need It
Performance is an engineering hypothesis to test.
### Files Added
No benchmark framework by default.
### Files Changed
None in service.
### Rust Concepts Used
Allocations, borrowing, release optimization.
### Tests Added
Keep behavior tests; record baseline as exercise output.
### Architecture Impact
No complexity added without a measured need.

## Day Summary
Measure representative behavior, profile to find causes, then optimize and re-measure.

## What You Should Be Able to Do Now
Design a benchmark that states its workload and limitations.
