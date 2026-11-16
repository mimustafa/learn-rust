# Day 20 — Timeouts, cancellation, and bounded async work

**Time box:** 60 minutes.

## Learning Objectives
- Apply a timeout and reason about dropping a future.
- Identify how connection pools and request limits bound service work.

## Why This Matters
An unbounded wait or concurrency level can exhaust sockets, memory, or database capacity.

## Concepts
Timeouts bound waiting, not necessarily every downstream side effect. Dropping a future cancels local polling; a request already sent to a database may need separate semantics. Never hold a synchronous lock across `.await`.

Async work is cooperative: CPU loops can starve an executor. Use `spawn_blocking` or a bounded CPU worker pool for genuinely blocking/CPU-heavy work, not as a substitute for normal async I/O. Async-aware locks/channels exist, but reducing shared mutable state is usually simpler. Structured task ownership and explicit shutdown keep spawned work observable.

## Explanation
The service uses a 10-second HTTP timeout, a pool max of 10, and a 5-second pool acquire timeout. Each protects a different resource boundary.

## Main Project
Review the timeout layer and pool options. Propose a separate DB statement timeout for a production deployment.

## Coding Exercises
1. Wrap a future in `tokio::time::timeout`; test both a future that finishes and one that times out.

## Quiz
1. Does cancelling a local future prove a remote side effect did not happen?
2. Why not hold a standard mutex guard across `.await`?

## Test Your Understanding
Name one limit for inbound concurrency and one for database connections.

## Reference Solutions
See instructor exercise solution Day 20.

## Common Mistakes
Unbounded `spawn`; retrying non-idempotent writes blindly; assuming timeout undoes work.

## Production Perspective
Align timeouts across client, proxy, handler, pool, and database; budget retries to avoid amplification.

## Architecture Perspective
Failure semantics are part of API and persistence design, not just runtime tuning.

## Project Change
### What We Are Adding
Finite request and connection acquisition durations.
### Why We Need It
Avoid indefinite waits and unbounded resource occupation.
### Files Added
Timeout configuration in route composition.
### Files Changed
Pool initialization.
### Rust Concepts Used
Future cancellation, timers, error propagation.
### Tests Added
Complete and timeout paths in a Tokio test exercise.
### Architecture Impact
Limits are explicit; production needs coordinated downstream budgets.

## Day Summary
Cancellation and timeouts bound local work but require careful side-effect semantics.

## What You Should Be Able to Do Now
Choose a timeout boundary and explain its limits.
