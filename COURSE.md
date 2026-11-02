# Course guide

**Audience:** experienced developers new to Rust. **Duration:** exactly 30 sessions, approximately 60 minutes each. **Working style:** read the lesson, type the example, run the checks, attempt the exercises, then compare with the separated answer key. Examples in the final service use synthetic data only.

## Daily route

| Day | Learn | Project change | Time |
|---:|---|---|---:|
| 01 | Toolchain, Cargo, values, functions | First patient summary, no framework | 55 min |
| 02 | Expressions, control flow, `match` | Classify observation values | 55 min |
| 03 | Ownership, move, `Copy`, `Clone` | Decide which values the domain owns | 60 min |
| 04 | References, borrowing, slices | Borrow data in reporting functions | 60 min |
| 05 | Structs, enums, `Option`, `Result` | Model patient and observation states | 60 min |
| 06 | `String`, vectors, maps, sets | In-memory patient registry | 55 min |
| 07 | Closures and iterators | Filter and summarize observations | 55 min |
| 08 | Modules, visibility, Cargo packages | Split domain from application code | 60 min |
| 09 | Generics, traits, dispatch, newtypes | Add a focused repository boundary | 60 min |
| 10 | Lifetimes and API design | Read references without over-annotating | 60 min |
| 11 | Error propagation and custom errors | Validate inputs without panics | 60 min |
| 12 | Unit, integration, and async test strategy | Make domain behavior regression-safe | 60 min |
| 13 | Domain/application/infrastructure boundaries | Establish the modular-monolith shape | 60 min |
| 14 | Relational modeling, SQL, migrations | Define PostgreSQL tables and constraints | 60 min |
| 15 | SQLx, pools, queries, transactions | Persist patients and observations | 60 min |
| 16 | Axum, HTTP, JSON, validation | Expose versioned patient endpoints | 60 min |
| 17 | HTTP contracts, pagination, API tests | Bound list requests and normalize errors | 60 min |
| 18 | Configuration, authn/authz, data safety | Protect write/read routes with a dev bearer token | 60 min |
| 19 | Futures, Tokio, async I/O | Run the HTTP and database service asynchronously | 60 min |
| 20 | Timeouts, cancellation, bounded concurrency | Bound database and request work | 60 min |
| 21 | OS threads, channels, parallelism | Compare a bounded worker with async I/O | 55 min |
| 22 | `Send`, `Sync`, `Arc`, locks, atomics | Choose message passing before shared mutable state | 60 min |
| 23 | Background jobs, idempotency, backpressure | Design a bounded observation-processing queue | 60 min |
| 24 | `tracing`, health, shutdown | Add request-aware operational signals | 60 min |
| 25 | Benchmarking and profiling | Measure parsing before optimizing | 60 min |
| 26 | Reliability, rate limits, secure operations | Review resource limits and failure modes | 60 min |
| 27 | Architecture review, dependency direction | Keep the service modular without over-abstraction | 60 min |
| 28 | Service boundaries and distributed trade-offs | Decide whether observation processing should split | 60 min |
| 29 | Production-readiness exercise | Harden the single-service reference design | 60 min |
| 30 | Independent design and teach-back | Rebuild a feature and explain its trade-offs | 60 min |

The daily pages in [`days/`](days/) are the authoritative lessons. They keep each activity time-boxed; instructor answers are intentionally elsewhere.

## Project and milestones

The project is a synthetic patient/observation service. It starts as values and functions, evolves toward validated domain input, then a PostgreSQL-backed Axum API. The final runnable reference is [`project/final`](project/final/README.md); milestone notes describe the intermediate design decisions without maintaining redundant snapshots of every source file.

1. **Rust foundations (Days 1–4):** values, ownership, borrowing.
2. **Domain application (Days 5–8):** types, collections, iterators, modules.
3. **Types and tests (Days 9–13):** traits only at real seams, errors, tests, boundaries.
4. **PostgreSQL persistence (Days 14–15):** schema, migrations, SQLx.
5. **HTTP API (Days 16–18):** Axum, contracts, input checks, dev authentication.
6. **Async and concurrency (Days 19–23):** async I/O, bounded work, threads, coordination.
7. **Operations and performance (Days 24–26):** tracing, health, measurement, reliability.
8. **Architecture and independent practice (Days 27–30):** modular monolith, service boundaries, review.

## Ecosystem decisions

The reference service uses **Axum** for typed HTTP routing and extractors, **Tokio** for async I/O and shutdown, **SQLx** for explicit SQL and a PostgreSQL pool, **Serde** for JSON, **Chrono/UUID** for timestamps and identifiers, **thiserror** for application-facing error variants, **tracing** for structured diagnostics, **tower-http** for request tracing/timeouts, and **subtle** for constant-time token-content comparison. `dotenvy` is a local-development convenience only.

These are not mandatory prerequisites for learning Rust. Days 1–13 use the standard library first. SQLx is chosen over an ORM so SQL remains visible; SQLx runtime queries avoid requiring a live database at compile time. Axum is chosen over a larger framework because its Tower composition and extractor model expose useful Rust concepts. A real deployment needs an identity provider, TLS termination, secret management, observability export, migrations policy, and a privacy/security review; this sample intentionally does not claim to provide those.

The [`architecture/`](architecture/) decisions explain options and trade-offs. Versions are pinned by compatible major/minor ranges in `Cargo.toml`; refresh and audit dependencies before real deployments.

## Official references

- [The Rust Book](https://doc.rust-lang.org/book/) for the language progression.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) for runnable syntax examples.
- [The Rust Standard Library](https://doc.rust-lang.org/std/) for API behavior.
- [Cargo Book](https://doc.rust-lang.org/cargo/) for packages and build commands.
- [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/) for edition changes.

Use the stable documentation matching your installed toolchain when investigating version-specific APIs.
