# Day 26 — Reliability and resource limits

**Time box:** 60 minutes.

## Learning Objectives
- Identify resource boundaries and failure modes in the reference service.
- Explain timeouts, pool caps, bounded pages, and safe retries as separate controls.

## Why This Matters
Production reliability requires explicit limits; no single timeout or framework default solves resource exhaustion.

## Concepts
The sample caps DB connections, connection acquisition time, request duration, page size, and validates input. It lacks rate limiting, body-size policy, distributed tracing export, and deployment controls.

Also review dependency advisories, lockfile changes, provenance, and build inputs as supply-chain practices. Rust prevents many memory errors but does not prevent authorization mistakes, injection in unsafe query construction, or vulnerable dependencies.

## Explanation
Build a failure table: database unavailable at startup; DB lost after startup; invalid body; full pool; slow request; process shutdown. State expected response and alert/log signal.

## Main Project
Review every bound in code/config and compare it with expected workload and downstream budgets.

## Coding Exercises
1. Write a failure-mode table for three endpoints, including expected status, safe log context, and recovery action.

## Quiz
1. What does a DB pool limit not protect?
2. Why can automatic retries amplify an outage?

## Test Your Understanding
Name a missing control and the operational owner needed to configure it safely.

## Reference Solutions
See instructor exercise solution Day 26.

## Common Mistakes
Claiming the sample is production-ready; retrying every error; exposing dependency details publicly.

## Production Perspective
Real service readiness includes SLOs, alerting, privacy/security review, backup/restore, migrations, TLS, identity, and incident ownership.

## Architecture Perspective
Reliability controls compose; each should address a threat/failure with an owner and test.

## Project Change
### What We Are Adding
A limit and failure-mode review.
### Why We Need It
Safety means considering resource exhaustion and dependency failure.
### Files Added
No deployment artifacts or CI/CD.
### Files Changed
Operational limitations in docs.
### Rust Concepts Used
Error surfaces, cancellation, bounded data.
### Tests Added
Exercise creates failure table; app tests cover local invariants.
### Architecture Impact
Documents boundaries rather than pretending tutorial code has full operations.

## Day Summary
Reliability depends on coordinated budgets, explicit failure policy, and operators.

## What You Should Be Able to Do Now
Trace a failure through API, handler, pool, database, and response.
