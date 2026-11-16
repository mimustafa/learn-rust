# Day 18 — Configuration and practical API security

**Time box:** 60 minutes.

## Learning Objectives
- Load required configuration without silent defaults for secrets.
- Distinguish authentication from authorization and identify the sample's security limits.

## Why This Matters
Healthcare context requires careful handling, but a tutorial token is not a production security system.

## Concepts
`DATABASE_URL` and `API_TOKEN` are required; bind address has a local default. Authentication answers who presented a credential; authorization answers what they may do. SQL uses bound parameters; logs and errors must not reveal secrets or patient data.

Never store passwords as plain text; password verification needs a vetted slow password hash (for example Argon2id) and credential lifecycle. JWTs are signed claims, not encrypted data; validate issuer, audience, expiry, signature, and key rotation rather than merely decoding them. TLS protects transport; rate limiting and request/body bounds reduce abuse. The sample intentionally implements none of these production identity controls.

## Explanation
Inspect `config.rs` and `authenticate` in `routes.rs`. `subtle` performs a constant-time comparison for equal-length token contents; matching length is checked first. The shared bearer token still has no user identity, rotation, resource-level policy, or production secret handling. `subtle` is a small, audited primitive dependency; the standard library does not promise constant-time string comparison.

## Main Project
Run a protected route without a token and with an incorrect token. Confirm neither token appears in logs.

## Coding Exercises
1. Add a test of the auth decision as a pure helper, or document why extracting a helper is preferable to constructing full service state.

## Quiz
1. Does authentication prove a caller may view every patient?
2. Which environment value must never have a code default?

## Test Your Understanding
List three controls required before any real sensitive data could be considered.

## Reference Solutions
See instructor exercise solution Day 18.

## Common Mistakes
Committing `.env`; logging Authorization headers; calling a shared token full authorization.

## Production Perspective
Use an identity provider, TLS, least privilege, key rotation, audit policy, rate limiting, and threat/privacy review. This sample is not a PHI system.

## Architecture Perspective
Security controls must be designed with identity, resource ownership, and operator capabilities—not bolted on by naming a middleware.

## Project Change
### What We Are Adding
Required env config and protected route layer.
### Why We Need It
Teach configuration and auth boundaries with explicit limitations.
### Files Added
`.env.example`, `config.rs`, auth middleware.
### Files Changed
Routes and README safety notes.
### Rust Concepts Used
Environment parsing, `Result`, shared `Arc<str>`, middleware.
### Tests Added
Manual unauthenticated/authorized smoke checks.
### Architecture Impact
Local bearer-token gate applies consistently to `/v1`.

## Day Summary
Never mistake a teaching token for production identity and authorization.

## What You Should Be Able to Do Now
Explain the sample's security boundary and name what it does not protect.
