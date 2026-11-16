# Day 15 — SQLx, pools, and parameterized queries

**Time box:** 60 minutes.

## Learning Objectives
- Execute a parameterized query with SQLx.
- Explain connection pooling and why user input must never be SQL text.

## Why This Matters
Persistence is an I/O boundary with failures, resource limits, and injection risk.

## Concepts
SQLx provides async PostgreSQL access and row decoding. A pool caps reusable connections. `.bind(value)` keeps data separate from SQL syntax (prepared/parameterized execution). Runtime query APIs keep builds independent of a live DB but require integration tests. A transaction groups related statements; commit only after all succeed, otherwise rollback. Isolation level controls which concurrent changes are visible; choose it from consistency requirements rather than habit.

## Explanation
Read `list_patients` and `create_patient` in `src/routes.rs`. The query text is fixed; request values are bound. `PgPoolOptions` sets a maximum and acquisition timeout.

## Main Project
Use migration startup, a bounded pool, a stable sort, and a typed `FromRow` result.

## Coding Exercises
1. Implement a SQLx query selecting one patient by UUID using `.bind(id)` and `fetch_optional`; distinguish not-found from a database error.

## Quiz
1. What does a pool bound?
2. Why is `format!("... {user_input}")` unsafe for SQL?

## Test Your Understanding
Describe when a multi-statement transaction is necessary and how an error should affect commit.

## Reference Solutions
See instructor exercise solution Day 15.

## Common Mistakes
Ignoring row errors; unbounded connections; assuming runtime query compilation checks schema.

## Production Perspective
Test migrations and queries against disposable PostgreSQL; consider transaction isolation and query plans.

## Architecture Perspective
Keep SQL visible while avoiding an abstraction layer that adds no current value.

## Project Change
### What We Are Adding
Pool, startup migration, typed SQL reads/writes.
### Why We Need It
Patients and observations must persist beyond process memory.
### Files Added
`main.rs`, migration and route query code.
### Files Changed
`Cargo.toml`, `state.rs`, `routes.rs`.
### Rust Concepts Used
Async functions, `Result`, `FromRow`, ownership at I/O boundaries.
### Tests Added
Fast unit tests remain database-independent; integration work uses disposable PostgreSQL.
### Architecture Impact
Routes coordinate database calls; pure validation remains in domain.

## Day Summary
Bind values, bound the pool, and treat database operations as fallible async I/O.

## What You Should Be Able to Do Now
Write a parameterized SQLx query and explain its error/absence behavior.
