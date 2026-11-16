# Day 14 — Relational modeling and migrations

**Time box:** 60 minutes.

## Learning Objectives
- Map domain relationships to keys and constraints.
- Explain why migrations are versioned source code.

## Why This Matters
The database must enforce integrity even when a request path or application bug misses a check.

## Concepts
Patient has many observations; a foreign key expresses that relationship. A primary key identifies a row. `NOT NULL`, `CHECK`, and indexes enforce/query known invariants. Use migration files rather than editing a live schema manually.

## Explanation
Inspect `migrations/0001_initial.sql`. The observations index matches patient-scoped, newest-first retrieval. SQL constraints complement, not replace, validation.

## Main Project
Review table columns, UUID identity, observation foreign key, finite numeric values, and cascade behavior.

## Coding Exercises
1. Write a migration for a note table linked to a patient with a non-empty bounded body; add one index justified by a query.

## Quiz
1. Which mechanism prevents observations referencing nonexistent patients?
2. Why should a migration be committed with code?

## Test Your Understanding
Explain whether the index helps a query filtering only on `code`.

## Reference Solutions
See instructor exercise solution Day 14.

## Common Mistakes
Duplicating every application check without thinking; adding indexes without a query; storing sensitive values casually.

## Production Perspective
Schema changes need rollout/rollback planning and data retention/privacy decisions beyond this tutorial.

## Architecture Perspective
SQL schema is part of the domain contract and operational lifecycle.

## Project Change
### What We Are Adding
PostgreSQL tables, constraints, and patient/time index.
### Why We Need It
Persistence and relational integrity are core service requirements.
### Files Added
`migrations/0001_initial.sql`.
### Files Changed
Reference service schema.
### Rust Concepts Used
Types map to SQL; migrations are external resources loaded by the application.
### Tests Added
Inspect migration; database integration tests are a follow-up.
### Architecture Impact
Persistence remains explicit SQL rather than ORM magic.

## Day Summary
Schema constraints protect shared integrity; indexes must follow actual access paths.

## What You Should Be Able to Do Now
Describe a one-to-many schema and write a migration with a justified index.
