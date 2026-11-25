# Daily quiz answer key

## Days 1–10
1. `let` bindings are immutable unless `mut`; a function's last expression is returned without `;`.
2. `match` must cover all possible variants/values; `_` is a catch-all.
3. A move transfers ownership; `Copy` creates a bitwise copy for eligible small values and leaves the source usable.
4. A shared `&T` cannot coexist with a live `&mut T` to the same value; references cannot outlive their referent.
5. `Option<T>` expresses presence/absence; `Result<T,E>` expresses success/failure.
6. `String` owns a growable UTF-8 buffer; `Vec<T>` owns a growable sequence.
7. `iter()` borrows, `iter_mut()` mutably borrows, `into_iter()` consumes the collection.
8. A package can contain one or more crates; a module organizes names and visibility within a crate.
9. Generics use static dispatch unless a trait object is used; trait bounds state required behavior.
10. A lifetime annotation relates references; it does not allocate or prolong a value.

## Days 11–20
11. `?` returns an error early, converting it when `From` supports conversion.
12. Unit tests can access private items in their module; integration tests use the public crate API.
13. Domain rules should not depend on an HTTP request type or SQL connection.
14. Bind user values as SQL parameters; never interpolate them into SQL text.
15. A pool reuses bounded database connections; transactions group operations with an explicit commit/rollback outcome.
16. HTTP status and JSON shape are part of the API contract; validate before persistence.
17. Pagination needs a server-enforced maximum to bound work and response size.
18. Authentication identifies a caller; authorization decides what that caller may do. A shared token does not provide patient-level authorization.
19. An async function returns a future; it runs when polled by an executor.
20. Timeouts bound waiting; cancellation means a future may be dropped before completion, so side effects need careful design.

## Days 21–30
21. Concurrency is overlapping progress; parallelism is simultaneous execution on multiple cores.
22. `Send` permits transfer across threads; `Sync` permits shared references across threads.
23. A bounded queue creates backpressure; an unbounded queue can exhaust memory.
24. Structured `tracing` fields carry context; never include secrets or unnecessary health data.
25. Benchmark representative release-mode behavior, repeat measurements, and profile before optimizing.
26. Readiness reports ability to serve dependencies; liveness reports that the process is alive.
27. Dependency direction should keep domain rules independent of transport/storage details.
28. A service split adds network failures, deployment, observability, and contract/versioning costs.
29. There is no single correct hardening list; justify each control by threat, risk, and operator capacity.
30. A successful teach-back explains ownership decisions, test evidence, error behavior, limits, and architecture trade-offs.
