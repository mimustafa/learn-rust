# Day 12 — Testing Rust code

**Time box:** 60 minutes.

## Learning Objectives
- Write unit tests in a module and run selected tests.
- Explain the difference between unit and integration test boundaries.

## Why This Matters
Tests make domain refactoring safe before the database and HTTP layers arrive.

## Concepts
`#[test]` marks synchronous tests; `assert_eq!` compares values. Unit tests can access private module items in a child test module. Integration tests in `tests/` use the public crate API. Async tests use the chosen runtime macro. Prefer explicit fixtures/builders for setup; use a fake or mock only at a meaningful boundary. Property tests explore generated inputs; fuzzing explores malformed input at scale; concurrency tests probe interleavings. Those techniques complement, not replace, readable examples and deterministic unit tests.

Coverage can identify untested branches but does not measure test quality. A DB-backed suite should use disposable isolated data and explicit setup/cleanup; a missing database is a setup failure, not a reason to report a skipped test as success.

## Explanation and Runnable Example
```rust
fn normalize(input: &str) -> String { input.trim().to_owned() }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trims_whitespace() { assert_eq!(normalize(" Ada "), "Ada"); }
}
fn main() {}
```
Run `cargo test`; test names are executable documentation.

## Main Project
The final service tests name/measurement validation and pagination limits without requiring PostgreSQL.

## Coding Exercises
1. Add tests for empty and overlong names. Make the rule explicit before implementation.

## Quiz
1. Where can a unit test access a private function?
2. Do all tests need an external database?

## Test Your Understanding
Break a boundary assertion and observe how Cargo reports the failing test.

## Reference Solutions
See instructor exercise solution Day 12.

## Common Mistakes
Testing only happy paths; tests coupled to implementation details; hidden database prerequisites.

## Production Perspective
Use integration tests with a disposable database for SQL/schema behavior; never silently skip required coverage.

## Architecture Perspective
Pure functions are cheap to test and reduce the need for mocks.

## Project Change
### What We Are Adding
Regression tests for domain rules and page bounds.
### Why We Need It
The compiler cannot prove business validation is correct.
### Files Added
`#[cfg(test)]` modules.
### Files Changed
Domain and routes.
### Rust Concepts Used
Attributes, test modules, assertions.
### Tests Added
Name, finite values, codes, pagination.
### Architecture Impact
Pure rules remain independent of I/O.

## Day Summary
Tests exercise behavior; keep the fast suite independent of external services.

## What You Should Be Able to Do Now
Turn an input boundary requirement into a focused failing/passing unit test.
