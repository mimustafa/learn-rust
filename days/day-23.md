# Day 23 — Background work, idempotency, and backpressure

**Time box:** 60 minutes.

## Learning Objectives
- Design a bounded producer/consumer queue.
- Explain idempotency and retry hazards for writes.

## Why This Matters
Background jobs turn request work into eventual work but introduce queue failure and shutdown semantics.

## Concepts
A bounded channel makes producers wait when full. A durable queue survives process loss; an in-memory Tokio channel does not. Idempotency keys help clients safely retry when they cannot know whether a timed-out write committed.

## Explanation
The service intentionally does not launch a background worker; this day designs one before adding operational machinery.

## Main Project
Sketch a queue for observation normalization. Define capacity, duplicate behavior, retry budget, and shutdown policy before implementation.

## Coding Exercises
1. Create a bounded Tokio channel, send two values, and receive them; explain what backpressure means when capacity is reached.

## Quiz
1. What risk does an unbounded channel create?
2. Why is retrying a create request after timeout ambiguous?

## Test Your Understanding
Would a process-local queue meet a requirement that work survive a restart?

## Reference Solutions
See instructor exercise solution Day 23.

## Common Mistakes
Fire-and-forget unbounded spawning; claiming in-memory queues are durable; retrying non-idempotent operations.

## Production Perspective
Durability, visibility timeout, dead-letter handling, deduplication, and operator recovery need a real queue design.

## Architecture Perspective
Only add background processing when latency or workflow requirements demand it.

## Project Change
### What We Are Adding
A bounded worker design exercise, not a queue dependency.
### Why We Need It
Teach backpressure and failure semantics without hiding operations in a crate.
### Files Added
Lesson example.
### Files Changed
No production queue in the reference.
### Rust Concepts Used
Async channels, ownership transfer, cancellation.
### Tests Added
Queue capacity/closure exercise.
### Architecture Impact
Avoids an unjustified infrastructure component.

## Day Summary
Bound queues, define retry semantics, and distinguish volatile from durable work.

## What You Should Be Able to Do Now
Describe a safe producer/consumer boundary and its restart behavior.
