# Day 17 — HTTP contracts, pagination, and API testing

**Time box:** 60 minutes.

## Learning Objectives
- Define a stable response/error contract and bounded list operation.
- Design HTTP integration tests for status, headers, and body shape.

## Why This Matters
List endpoints can consume unbounded database and network resources without explicit limits.

## Concepts
The API defaults page size to 50 and caps it at 100. Stable ordering is required for repeatable paging. Offset pagination can drift under concurrent inserts; cursor pagination is a later alternative.

## Explanation
Review `page()` and the `ORDER BY` clauses in `routes.rs`. `ErrorBody` deliberately hides database internals. Database-backed endpoint tests should run against a disposable migrated database.

## Main Project
Try `limit=101`, `offset=-1`, and the default request. Define expected status and shape before testing.

## Coding Exercises
1. Add a unit test for `limit=100` acceptance and `limit=101` rejection; then describe what an isolated PostgreSQL integration test would add.

## Quiz
1. Why cap page size server-side?
2. What can change between offset pages?

## Test Your Understanding
Choose a cursor key for observations and explain its ordering semantics.

## Reference Solutions
See instructor exercise solution Day 17.

## Common Mistakes
Using unstable ordering; testing against shared mutable data; returning raw infrastructure errors.

## Production Perspective
Page caps, request timeouts, body limits, and DB pool size jointly control resource exposure.

## Architecture Perspective
Error/status shapes are public contracts; keep them stable while internal failures evolve.

## Project Change
### What We Are Adding
Validated pagination and normalized errors.
### Why We Need It
Bound work and make clients predictable.
### Files Added
Page query helper and error response.
### Files Changed
`routes.rs`, `error.rs`.
### Rust Concepts Used
Typed query extraction, `Option`, range patterns, error conversion.
### Tests Added
Page bounds and invalid offset unit tests.
### Architecture Impact
HTTP boundary owns query validation and representation.

## Day Summary
Bound list work, sort deterministically, and test the API contract.

## What You Should Be Able to Do Now
Design a bounded page endpoint and state its concurrency limitations.
