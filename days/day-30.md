# Day 30 — Independent feature and teach-back

**Time box:** 60 minutes.

## Learning Objectives
- Design and implement a small feature without copying a reference solution.
- Explain ownership, error behavior, testing, resource limits, and architecture choices.

## Why This Matters
The goal is independent engineering judgment, not completing a certificate or memorizing syntax.

## Concepts
Start from requirements and domain invariants. Select types; keep ownership explicit; define errors; write tests; use compiler/linter feedback; then review operational and security limitations.

## Explanation
Choose a synthetic-data feature, such as a new observation filter. Define input/output and failure rules before editing. Keep it bounded and avoid healthcare interpretation.

## Main Project
Implement the feature in the existing modular monolith. Do not create a new service unless you can defend it with evidence.

## Coding Exercises
1. Add a regression test proving `POST /v1/patients` rejects an unknown JSON field with `400` before database access. Then write a short review note explaining the input contract and one limitation.

## Quiz
1. What makes a Rust abstraction justified?
2. What does a passing test suite not prove?

## Test Your Understanding
Teach another developer one ownership error and its design-level fix in two minutes.

## Reference Solutions
The reference route test is in `project/final/src/routes.rs`; the instructor guide includes the test shape and review rubric.

## Common Mistakes
Copying a solution without reasoning; feature creep; claiming complete production competence after one course.

## Production Perspective
Continue with code review, integration testing, real profiling, deployment practice, and supervised system ownership.

## Architecture Perspective
State the simplest design that satisfies the requirement and the evidence that would justify evolution.

## Project Change
### What We Are Adding
A learner-owned feature and teach-back.
### Why We Need It
Demonstrate independent reasoning, not answer matching.
### Files Added
One focused feature/test and review note in your work log.
### Files Changed
Learner-selected service module.
### Rust Concepts Used
Ownership, types, errors, tests, compiler feedback.
### Tests Added
Feature behavior and failure boundary.
### Architecture Impact
Review one alternative and its cost.

## Day Summary
Rust competence grows through independent implementation, explanation, review, and measurement.

## What You Should Be Able to Do Now
Build and defend a modest Rust service change, while recognizing what requires further production experience.
