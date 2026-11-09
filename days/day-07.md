# Day 07 — Closures and iterators

**Time box:** 55 minutes.

## Learning Objectives
- Distinguish `iter`, `iter_mut`, and `into_iter`.
- Build a readable iterator pipeline without losing ownership clarity.

## Why This Matters
Iterator adapters express data transformations and often avoid intermediate collections.

## Concepts
`iter()` yields shared references; `iter_mut()` yields mutable references; `into_iter()` consumes the collection. Adapters such as `filter` and `map` are lazy until a consumer (`collect`, `sum`, `find`) runs.

## Explanation and Runnable Example
```rust
fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}
fn main() {
    let observations = [60.0, 72.0, 80.0];
    assert_eq!(mean(&observations), Some(212.0 / 3.0));
}
```

## Main Project
Summarize borrowed observation values without consuming their records.

## Coding Exercises
1. From `&[i32]`, return the sum of even values using `iter`, `filter`, and `map` or `sum`; test an empty slice.

## Quiz
1. Which iteration form consumes a vector?
2. When does a lazy iterator pipeline execute?

## Test Your Understanding
Change `iter()` to `into_iter()` and predict whether the original collection remains usable.

## Reference Solutions
See instructor exercise solution Day 07.

## Common Mistakes
Collecting only to iterate again; accidentally moving values; making a pipeline obscure.

## Production Perspective
Readable iteration usually beats manual indexing; benchmark only if a measured hotspot warrants it.

## Architecture Perspective
Iterator types communicate whether a function reads, mutates, or consumes data.

## Project Change
### What We Are Adding
Pure aggregation over borrowed measurements.
### Why We Need It
The service needs safe reporting without ownership transfer.
### Files Added
Lesson example.
### Files Changed
None in service.
### Rust Concepts Used
Closures, iterator adapters/consumers, borrowing.
### Tests Added
Empty and mixed input cases.
### Architecture Impact
Small transformations can remain pure and independently testable.

## Day Summary
The iterator receiver determines ownership; adapters are lazy until consumed.

## What You Should Be Able to Do Now
Write and explain a borrowed iterator pipeline.
