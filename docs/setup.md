# Setup and validation

## Rust toolchain

The official stable channel manifest was checked on **2026-10-31**; it listed **Rust 1.99.1**, manifest date **2026-10-15**. The course targets **edition 2024** and selects the moving `stable` channel in `rust-toolchain.toml`. Edition is a crate-level language convention, not a compiler version.

Install rustup using the official instructions at <https://rustup.rs/>. Then:

```sh
rustup update stable
rustup component add rustfmt clippy
rustc --version
cargo --version
```

The authoring environment for this repository did not have `rustc` installed; a working compiler is required to run the validation commands below. The source targets stable Rust and Cargo, rather than relying on nightly features.

## PostgreSQL

Install PostgreSQL 16 or newer, create a local database/user, and set the variables shown in [`project/final/.env.example`](../project/final/.env.example). Copy that file to `.env` only for local use; `.env` is ignored by Git. Never put real patient data or production credentials in this sample.

```sh
cd project/final
cp .env.example .env
# Edit DATABASE_URL and API_TOKEN for your local instance.
cargo run
```

The service runs database migrations at startup. `DATABASE_URL` and a non-empty `API_TOKEN` are required; the sample does not provide a default secret. `GET /health/live` is unauthenticated. `GET /health/ready` checks database readiness. Other `/v1` routes require `Authorization: Bearer <API_TOKEN>`.

## Daily commands

From `project/final`:

```sh
cargo check
cargo test
cargo check --examples
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo run
```

`cargo check` catches type errors quickly; `cargo test` runs deterministic domain tests and does not require PostgreSQL. Endpoint/database integration tests require a disposable database and are an exercise, not silently skipped tests. Use `cargo build --release` only when measuring runtime behavior.

## HTTP smoke test

After startup, set the same local token in `TOKEN` and try:

```sh
curl -i http://127.0.0.1:3000/health/live
curl -i http://127.0.0.1:3000/v1/patients \
  -H "Authorization: Bearer $TOKEN"
curl -i -X POST http://127.0.0.1:3000/v1/patients \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"full_name":"Example Patient","birth_date":"1980-04-12"}'
```

The sample is intentionally not suitable for real healthcare records, deployment, or clinical decisions.
