# Day 29 — Production-oriented review

**Time box:** 60 minutes.

## Learning Objectives
- Audit one endpoint for correctness, resource bounds, privacy, and observability.
- Turn a risk into a specific mitigation and verification test.

## Why This Matters
Rust memory safety does not automatically make application policy, privacy, auth, or operations correct.

## Concepts
Review validation, authorization scope, SQL parameterization, error detail, logging, limits, timeout behavior, data retention, and shutdown. Dependency audits and TLS/secrets policy are operational practices, not compiler guarantees.

Also consider authentication/session lifecycle, rate limiting, secure transport, least privilege, secret rotation, audit needs, and how an operator verifies dependency integrity. Never use these omissions as a reason to handle real health data in the sample.

## Explanation
Use a checklist on `POST /v1/patients`. Ask whether the system's shared token represents a real user; answer: no.

## Main Project
Pick one gap that can be described without pretending the sample is clinical software. Implement a focused test or document a bounded follow-up.

## Coding Exercises
1. Add a threat→mitigation→verification row for malformed input, missing credential, DB outage, and sensitive log leakage.

## Quiz
1. Does Rust's type safety prove endpoint authorization is correct?
2. Where should operational secrets be managed?

## Test Your Understanding
Could the application distinguish which user accessed which patient? What follows from the answer?

## Reference Solutions
See instructor exercise solution Day 29.

## Common Mistakes
Equating safe memory with secure system; declaring production readiness from a passing unit test.

## Production Perspective
Use dedicated privacy/security engineering and deployment review before real data. This course project is not suitable for PHI.

## Architecture Perspective
Controls should map to risks and be testable by the people who operate the system.

## Project Change
### What We Are Adding
A bounded hardening review and learner-selected verification.
### Why We Need It
Bring Rust correctness into system-level engineering context.
### Files Added
Test or review record only.
### Files Changed
Learner chooses a surgical change.
### Rust Concepts Used
Validation, errors, tests, ownership of secrets.
### Tests Added
Focused test for selected risk.
### Architecture Impact
No claim of production readiness; gaps remain explicit.

## Day Summary
Language safety is one layer; system security and operations require additional controls.

## What You Should Be Able to Do Now
Convert a production risk into an owned, verifiable engineering action.
