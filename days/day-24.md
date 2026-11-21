# Day 24 — `tracing`, health checks, and shutdown

**Time box:** 60 minutes.

## Learning Objectives
- Emit structured events with useful context.
- Distinguish liveness from readiness and explain graceful shutdown.

## Why This Matters
An operator needs to tell whether a process is alive, able to serve, or failing on a dependency.

## Concepts
`tracing` represents structured spans/events and works across async tasks. Do not log secrets, auth headers, bodies, or unnecessary personal/health data. Liveness asks whether the process runs; readiness checks required dependencies.

Use levels (`trace`, `debug`, `info`, `warn`, `error`) intentionally and attach safe structured fields, not formatted blobs. Metrics summarize counts/latency; traces connect operations; logs explain events. A request ID can correlate work, but never use a patient identifier as a substitute.

## Explanation
The service initializes a subscriber once, uses tower request tracing, exposes `/health/live` and DB-checking `/health/ready` (`503` when unavailable), and waits for Ctrl-C before shutdown.

## Main Project
Run each health endpoint with PostgreSQL available and unavailable; note startup failure and ready behavior.

## Coding Exercises
1. Add a span around one operation with a static operation name and a synthetic request ID; ensure no token/body is recorded.

## Quiz
1. Should liveness require a working database?
2. What context is safe to add to a log event?

## Test Your Understanding
What does graceful shutdown need to do with requests already in progress?

## Reference Solutions
See instructor exercise solution Day 24.

## Common Mistakes
Logging full request structs; reporting ready while required dependencies are unavailable; swallowing signal errors.

## Production Perspective
Add metrics/export, alerts, and privacy review only with clear operational ownership.

## Architecture Perspective
Observability is cross-cutting infrastructure; use structured context without coupling domain behavior to a vendor.

## Project Change
### What We Are Adding
Tracing initialization, health endpoints, graceful shutdown.
### Why We Need It
Operators need bounded, actionable runtime signals.
### Files Added
Tracing setup and health handlers.
### Files Changed
`main.rs`, `routes.rs`.
### Rust Concepts Used
Async signal handling, structured fields, state extraction.
### Tests Added
Manual live/ready checks; add endpoint integration tests with DB.
### Architecture Impact
Operational concerns remain at the application edge.

## Day Summary
Separate process liveness from dependency readiness; keep telemetry privacy-safe.

## What You Should Be Able to Do Now
Design health checks and a shutdown path that communicate actual service state.
