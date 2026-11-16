# Day 13 — Separate domain decisions from I/O

**Time box:** 60 minutes.

## Learning Objectives
- Distinguish domain, application, and infrastructure concerns.
- Keep validation independent of Axum and PostgreSQL.

## Why This Matters
A feature grows more safely when rules can be tested without booting a server or database.

## Concepts
The domain owns language and invariants; application flow coordinates work; infrastructure talks to SQL/HTTP. A single package and a few modules are enough initially. Clean/hexagonal architecture are ideas, not a requirement to create layers for every function.

## Explanation
Trace `POST /v1/patients`: JSON extraction → `validate_name` → parameterized insert → response/error mapping. Validation is pure; the handler owns transport and database coordination.

## Main Project
Review `src/domain.rs`, `routes.rs`, and `error.rs`. Draw arrows showing which module imports which.

## Coding Exercises
1. Move a name-length check from a toy handler into a pure `validate_name(&str)` function; test it without constructing HTTP state.

## Quiz
1. Should domain validation return `StatusCode`?
2. Why can starting with many layers hurt?

## Test Your Understanding
Propose one actual reason to extract a repository trait in this project.

## Reference Solutions
See instructor exercise solution Day 13.

## Common Mistakes
Premature dependency injection; domain functions importing transport types.

## Production Perspective
An explicit boundary makes security checks and validation easier to audit.

## Architecture Perspective
Patterns should solve current coupling/testability problems and justify their ongoing cost.

## Project Change
### What We Are Adding
A small modular-monolith boundary.
### Why We Need It
Database and HTTP concerns arrive next.
### Files Added
`domain`, `routes`, `error`, `state`.
### Files Changed
Application shape documented in ADR-002.
### Rust Concepts Used
Modules, visibility, composition, concrete types.
### Tests Added
Domain tests do not need I/O.
### Architecture Impact
Domain logic stays independent of framework and storage.

## Day Summary
Start with clear modules and add abstractions only when a real seam appears.

## What You Should Be Able to Do Now
Place a new rule in the module that owns its meaning.
