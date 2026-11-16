# Day 16 — Axum routes and typed JSON

**Time box:** 60 minutes.

## Learning Objectives
- Describe how a request extractor becomes a typed handler parameter.
- Implement a JSON create endpoint with validation and an appropriate status.

## Why This Matters
HTTP is a boundary: untrusted bytes become typed values only after parsing and validation.

## Concepts
Axum composes route handlers, extractors, state, and middleware. `Json<T>` deserializes; it does not prove a business invariant. Return `201 Created` after successful creation and stable errors for invalid input.

## Explanation
Follow `create_patient`: extract JSON, validate name, bind values to SQL, return a typed JSON response. A JSON parse rejection is normalized as a bad request.

## Main Project
The reference exposes `/v1/patients`; no handler should accept a raw SQL fragment or trust a deserialized string.

## Coding Exercises
1. Add an `email` field only if you can define validation and migration behavior; otherwise explain why the current domain omits it.

## Quiz
1. Does deserialization validate a patient's name?
2. Which status indicates a resource was created?

## Test Your Understanding
Trace malformed JSON separately from a well-formed but invalid name.

## Reference Solutions
See instructor exercise solution Day 16.

## Common Mistakes
Treating JSON parsing as domain validation; returning database error strings; overloading handlers.

## Production Perspective
Specify request-size limits, schema evolution, and API compatibility before exposing data.

## Architecture Perspective
Handlers translate transport input to domain operations; they should not own domain policy.

## Project Change
### What We Are Adding
Versioned patient/observation HTTP routes.
### Why We Need It
The service needs a practical API boundary.
### Files Added
Axum route definitions and DTOs.
### Files Changed
`routes.rs`, dependencies.
### Rust Concepts Used
Extractors, async handlers, `Result`, Serde.
### Tests Added
Manual smoke tests; domain tests verify validation.
### Architecture Impact
HTTP is an adapter around domain rules and SQL.

## Day Summary
Deserialize, validate, persist, and respond as distinct steps.

## What You Should Be Able to Do Now
Explain what Axum extracts and what the application must still validate.
