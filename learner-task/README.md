# Learner programs

Each `day-NN` folder contains a small, runnable example that follows the matching lesson in [`../days/`](../days/). Most examples use only Rust's standard library and can be compiled directly with `rustc`. Days 19, 20, and 23 use Tokio to demonstrate async tasks, timeouts, and bounded async channels; Day 30 uses Serde to demonstrate strict JSON request fields.

Run one day at a time from the repository root. For example:

```sh
rustc learner-task/day-01/main.rs -o /tmp/day-01 && /tmp/day-01
rustc learner-task/day-02/main.rs -o /tmp/day-02 && /tmp/day-02
rustc learner-task/day-03/main.rs -o /tmp/day-03 && /tmp/day-03
rustc learner-task/day-04/main.rs -o /tmp/day-04 && /tmp/day-04
```

Use the corresponding day number for Days 5–18, 21–29. For example, Day 12's tests run with:

```sh
rustc --test learner-task/day-12/main.rs -o /tmp/day-12-tests && /tmp/day-12-tests
```

Run the examples that need Cargo dependencies from the repository root:

```sh
cargo run --manifest-path learner-task/day-19/Cargo.toml
cargo run --manifest-path learner-task/day-20/Cargo.toml
cargo run --manifest-path learner-task/day-23/Cargo.toml
cargo run --manifest-path learner-task/day-30/Cargo.toml
cargo test --manifest-path learner-task/day-30/Cargo.toml
```

The Cargo commands may download their declared crates the first time they run. These are lesson examples, not a production health service; use synthetic data only.
