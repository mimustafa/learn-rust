# Day 28 — Service boundaries and distributed trade-offs

**Time box:** 60 minutes.

## Learning Objectives
- Identify a candidate service boundary from independent ownership/scaling needs.
- Explain network failure, idempotency, timeout, retry, and observability implications.

## Why This Matters
Microservices move complexity from in-process calls into distributed operations.

## Concepts
REST and gRPC are contract choices; neither makes a service boundary correct. Retries need budgets and idempotent semantics. Eventual consistency and duplicate delivery affect data behavior. A modular monolith can preserve boundaries without deployment overhead.

REST/JSON is broadly inspectable and easy for public APIs; gRPC/protobuf can provide typed contracts and efficient internal RPC but adds code generation and tooling. Async messaging decouples timing but introduces delivery, ordering, and deduplication semantics. Every network call needs timeout and bounded retry decisions.

## Explanation
Use patient identity and observation ingestion as a hypothetical split. Ask whether independent scaling, team ownership, or data isolation justifies duplicating operations and failure handling.

## Main Project
Do not implement another network service. Record a decision in line with ADR-006.

## Coding Exercises
1. Write a one-page boundary proposal containing requirement, owner, contract, failure model, data ownership, and a reason to keep one service.

## Quiz
1. Name two costs of an RPC boundary.
2. Why must a retry-safe command be idempotent or deduplicated?

## Test Your Understanding
Would a second process improve this course project today? State measurable evidence needed.

## Reference Solutions
See instructor exercise solution Day 28.

## Common Mistakes
Splitting by entity/table; ignoring network partitions; treating retries as free.

## Production Perspective
Distributed tracing, independent deployment, secrets, service discovery, failure budgets, and on-call ownership are real costs.

## Architecture Perspective
Extract only when a boundary improves an important quality attribute enough to pay for operations.

## Project Change
### What We Are Adding
A service-boundary decision, not a deployment.
### Why We Need It
Teach microservice judgment without premature infrastructure.
### Files Added
ADR-006 decision.
### Files Changed
No service split.
### Rust Concepts Used
Types/contracts remain local; network codecs are not introduced without need.
### Tests Added
Boundary proposal review.
### Architecture Impact
The modular monolith remains the reference.

## Day Summary
Service count is not an architecture quality metric.

## What You Should Be Able to Do Now
Explain what evidence and operational ownership are required to split a service.
