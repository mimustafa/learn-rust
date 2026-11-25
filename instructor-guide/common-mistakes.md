# Common learner mistakes

| Mistake | Coaching response |
|---|---|
| Clone until the borrow checker is quiet | Ask who owns the value and whether borrowing suffices; clone only for an intentional independent copy |
| Treat `String` and `&str` as interchangeable | Explain owned buffer versus borrowed UTF-8 view and select API ownership intentionally |
| Call `unwrap()` on request/database data | Identify the failure boundary and return a typed error or validate before use |
| Add a lifetime annotation to every reference | Remove annotations and let elision apply; add one only to express a real relationship |
| Create a trait for every struct | Require a concrete substitution/testing need; otherwise keep a concrete type |
| Use `async` for CPU parallelism | Compare cooperative waiting with worker threads; bound CPU work separately |
| Put a standard mutex around async code without thought | Consider message passing or an async-aware lock; never hold a synchronous lock across `.await` |
| Spawn unbounded tasks | Ask what caps in-flight work, memory, and downstream connections |
| Build SQL by formatting user input | Use bind parameters and database constraints |
| Return database error text to clients | Log contextual internal detail safely and return a stable generic error |
| Log request bodies or tokens | Treat them as sensitive; log IDs/correlation fields only when safe and justified |
| Split into microservices because the course mentions them | Demand an independent scaling, ownership, or reliability need and account for network failure |
| Call health success without checking dependencies | Distinguish liveness (process exists) from readiness (can serve required work) |
