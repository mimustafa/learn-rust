# Day 19 — Futures, async, and Tokio

**Time box:** 60 minutes.

## Learning Objectives
- Explain that calling an `async fn` creates a future.
- Distinguish cooperative I/O concurrency from OS-thread parallelism.

## Why This Matters
HTTP and database operations spend much time waiting; async lets a runtime progress other tasks during waits.

## Concepts
An `async fn` returns a future. An executor polls it; `.await` may yield. Async does not automatically create a thread or make CPU-heavy work faster. Axum and SQLx use Tokio in the reference.

## Explanation and Runnable Example
```rust
async fn answer() -> u32 {
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    42
}
#[tokio::main]
async fn main() {
    println!("{}", answer().await);
}
```
The macro sets up a runtime and block-on entry point. Tokio is an explicit dependency in the service.

## Main Project
Trace `main` → listener → async handler → SQLx await. Identify each I/O wait.

## Coding Exercises
1. Write a Tokio task that awaits a short timer and returns a value; compare it with a blocking sleep.

## Quiz
1. Does an async function execute on a new OS thread by itself?
2. What drives a future toward completion?

## Test Your Understanding
Explain why blocking sleep in an async handler prevents that worker from polling other ready tasks.

## Reference Solutions
See instructor exercise solution Day 19.

## Common Mistakes
Assuming `.await` means parallel execution; spawning a task for every tiny operation.

## Production Perspective
Runtime worker starvation and task resource use need metrics and limits.

## Architecture Perspective
Choose async because dependencies and workload are I/O-bound, not because it is fashionable.

## Project Change
### What We Are Adding
Tokio runtime, async listener and database calls.
### Why We Need It
HTTP + PostgreSQL operations are I/O-bound.
### Files Added
Async main and route functions.
### Files Changed
Runtime/database dependencies.
### Rust Concepts Used
Futures, `async`, `.await`, executor.
### Tests Added
Async behavior is exercised in the service; pure test suite stays fast.
### Architecture Impact
The runtime is an infrastructure choice isolated to the application edge.

## Day Summary
Async is cooperative scheduling around futures, not automatic parallelism.

## What You Should Be Able to Do Now
Trace where a future is created, awaited, and driven by Tokio.
