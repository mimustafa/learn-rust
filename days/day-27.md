# Day 27 — Review architecture and dependency direction

**Time box:** 60 minutes.

## Learning Objectives
- Explain the modular-monolith choice and module dependency direction.
- Identify whether a proposed abstraction solves a concrete problem.

## Why This Matters
Architecture should lower change cost rather than increase pattern count.

## Concepts
Keep domain validation free of HTTP/SQL. The route layer composes transport, state, and SQL. Dependency inversion is useful when a stable policy needs to call replaceable detail; it does not require a repository trait for every query.

Hexagonal/clean architecture express dependency direction through ports and adapters; DDD helps name domain boundaries when the domain warrants it. For this small app, pure functions and concrete composition are often simpler. Avoid adding a trait, crate, or layer solely to match a diagram.

## Explanation
Read ADR-002 and draw current imports. For a test seam, first ask if a pure function or disposable database test is sufficient.

## Main Project
Trace a create-observation request from JSON DTO through validation, SQL, response, and failure mapping.

## Coding Exercises
1. Propose a change requiring a new module; state its responsibility, public API, and one reason not to create a trait.

## Quiz
1. Which direction should infrastructure dependencies point?
2. What evidence makes a new abstraction worthwhile?

## Test Your Understanding
Compare a concrete service struct to a trait-object service for this one-binary app.

## Reference Solutions
See instructor exercise solution Day 27.

## Common Mistakes
Architecture by fashionable diagram; giant generic repository; exposing internals through `pub`.

## Production Perspective
Clear ownership of migrations, API contracts, and operational responsibilities matters more than a pattern label.

## Architecture Perspective
Every boundary has a cost: indirection, types, wiring, and learning burden.

## Project Change
### What We Are Adding
An architecture review and one justified module boundary.
### Why We Need It
The project has enough layers to evaluate actual coupling.
### Files Added
No source change required.
### Files Changed
ADR review notes in learner's own commits.
### Rust Concepts Used
Module privacy, traits, composition.
### Tests Added
Run full local test/check suite.
### Architecture Impact
Preserve the simplest sufficient structure.

## Day Summary
Architecture is a set of reasoned boundaries, not a pattern checklist.

## What You Should Be Able to Do Now
Defend module placement and explain the complexity of an abstraction.
