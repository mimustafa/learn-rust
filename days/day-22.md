# Day 22 — `Send`, `Sync`, and shared state

**Time box:** 60 minutes.

## Learning Objectives
- Explain `Send` and `Sync` at a practical level.
- Choose message passing, atomics, mutexes, or `Arc` based on ownership needs.

## Why This Matters
Rust's type system prevents unsafe cross-thread sharing unless a type supports it.

## Concepts
`Send`: ownership can move between threads. `Sync`: `&T` can be shared across threads. `Arc<T>` shares ownership atomically; `Mutex<T>` coordinates exclusive mutation; atomics suit small independent state. Tokio offers async-aware synchronization.

Atomics are not a general replacement for locks: ordering and compound invariants matter. A `Mutex` can deadlock if lock ordering cycles; keep critical sections short. For async code use an async mutex only when a guard truly must cross an await, and prefer channels when one task can own the state.

## Explanation
The service state clones a `PgPool` handle and an `Arc<str>` token. It avoids shared mutable domain state; PostgreSQL owns persistence.

## Main Project
Inspect `AppState::clone`. Identify which values are cheaply shared and which state is not protected by a global mutex.

## Coding Exercises
1. Build a counter updated by two threads using `Arc<Mutex<u32>>`; scope the guard so it is dropped before any wait.

## Quiz
1. Is `Arc<T>` by itself a mutable-sharing primitive?
2. What is the distinction between `Send` and `Sync`?

## Test Your Understanding
When would an atomic counter be simpler than a mutex-protected structure?

## Reference Solutions
See instructor exercise solution Day 22.

## Common Mistakes
Wrapping everything in `Arc<Mutex<_>>`; holding locks through `.await`; poisoning/error behavior ignored.

## Production Perspective
Measure contention and keep critical sections short. Prefer a database transaction or single-owner worker when appropriate.

## Architecture Perspective
Shared state is a design choice; ownership and message flow can remove synchronization.

## Project Change
### What We Are Adding
Shared immutable service configuration and pool handle.
### Why We Need It
Handlers run concurrently and need access to shared services.
### Files Added
`state.rs`.
### Files Changed
Route state and initialization.
### Rust Concepts Used
`Arc`, `Send`, `Sync`, `Clone`.
### Tests Added
Compile-time bounds are exercised by runtime APIs; thread counter exercise.
### Architecture Impact
No global mutable cache is introduced.

## Day Summary
Share immutable handles cheaply; add synchronized mutation only when needed.

## What You Should Be Able to Do Now
Explain why a type can or cannot cross an async/thread boundary.
