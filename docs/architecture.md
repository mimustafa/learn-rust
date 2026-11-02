# Architecture through the course

## Domain

The system tracks a patient identifier and display name, plus observations attached to a patient. An observation has a code, numeric value, unit, and timestamp. It is a teaching model with synthetic data; it does not encode clinical interpretation, consent, provenance, or a healthcare interoperability standard.

## Evolution

| Stage | Design | What it teaches |
|---|---|---|
| Days 1–4 | Values, functions, borrowed report inputs | Syntax, ownership, and API choices before framework complexity |
| Days 5–8 | Domain structs/enums and modules | Types make invalid states harder to express; visibility controls boundaries |
| Days 9–13 | Concrete domain logic and a narrow repository seam | Use traits where substitution is needed; test domain behavior first |
| Days 14–18 | PostgreSQL, explicit SQL, HTTP handlers | Infrastructure adapts transport/storage without hiding SQL or validation |
| Days 19–26 | Tokio service with limits and structured telemetry | Async is cooperative I/O, not magic parallelism; bound resources |
| Days 27–30 | Modular monolith, explicit extraction decision | Distributed systems add failure and operational cost |

## Final reference layout

```text
src/
  config.rs       required runtime configuration
  domain.rs       validated input and pure rules
  error.rs        deliberate HTTP-safe error mapping
  routes.rs       HTTP handlers, extractors, middleware
  state.rs        shared pool and local development token
  main.rs         startup, migration, tracing, graceful shutdown
migrations/       PostgreSQL schema
```

The dependency direction is routes → domain and routes → state/infrastructure. Pure validation stays independent of SQL and HTTP. This small project does not need a large dependency-injection framework or a trait per database operation.

## Deliberate limitations

The service uses a single bearer token to teach authentication plumbing, not user identity or authorization. It has no patient-level access control, TLS termination, rate limiter, audit trail, key rotation, production secret manager, privacy review, or deployment automation. Add those only with real requirements, threat modeling, and operational ownership. Never use real patient information with this sample.
