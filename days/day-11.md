# Day 11 — `Result`, `?`, and useful errors

**Time box:** 60 minutes.

## Learning Objectives
- Propagate errors with `?`.
- Distinguish invalid user input, expected absence, and internal failure.

## Why This Matters
Backend failures are normal control flow; panics are not a client error contract.

## Concepts
`Result<T,E>` represents success/failure. `?` returns an error early and converts it when `From` supports conversion; `map_err` adds or changes context. `Option` is absence, not failure. `panic!` is for violated invariants/unrecoverable programming errors, not ordinary external input. `unwrap` panics; `expect` adds an invariant explanation. `thiserror` derives explicit error types useful at library/domain boundaries; `anyhow` is useful at an application top level when errors are reported rather than classified. The standard library is enough for simple cases.

## Explanation
```rust
fn parse_positive(input: &str) -> Result<u32, &'static str> {
    let value = input.parse::<u32>().map_err(|_| "not an integer")?;
    if value == 0 { return Err("must be positive"); }
    Ok(value)
}
fn main() { assert_eq!(parse_positive("4"), Ok(4)); }
```
Avoid `unwrap()` on request, environment, database, or file input. Use `expect()` only for a documented invariant.

## Main Project
`domain.rs` returns validation errors; `error.rs` maps them to HTTP status without exposing SQL detail.

## Coding Exercises
1. Parse a positive count, returning a distinct error for parse failure and zero.

## Quiz
1. What does `?` do on `Err`?
2. When is panic appropriate compared with `Result`?

## Test Your Understanding
Add a parse error context while preserving the original cause.

## Reference Solutions
See instructor exercise solution Day 11.

## Common Mistakes
Using `unwrap()` on fallible inputs; collapsing every failure into a misleading default.

## Production Perspective
Log safe operational context internally; return stable, non-sensitive error messages externally.

## Architecture Perspective
Translate errors at boundaries instead of teaching domain code about HTTP status codes.

## Project Change
### What We Are Adding
Typed validation and application errors.
### Why We Need It
Invalid input must never become a panic or success-shaped fallback.
### Files Added
`project/final/src/error.rs`.
### Files Changed
Domain validation and routes use `Result`.
### Rust Concepts Used
`Result`, `?`, conversion, `thiserror`.
### Tests Added
Valid, invalid, and boundary cases in domain tests.
### Architecture Impact
Separates internal cause from public HTTP error contract.

## Day Summary
Propagate failures explicitly and map them deliberately at the edge.

## What You Should Be Able to Do Now
Replace a panic-shaped input path with an explicit, testable error.
