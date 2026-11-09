# Day 01 — Start Rust; name the values

**Time box:** 55 minutes (10 setup, 15 learn/type, 20 exercise, 10 review).

## Learning Objectives
- Run `rustc`/Cargo and identify a package, binary crate, and edition.
- Read a `let` binding, infer its type, and write a function returning a `String`.

## Why This Matters
The compiler and Cargo are part of daily Rust development. We begin with useful domain data, not a toy unrelated to the service.

## Concepts
Rust has immutable bindings by default; `mut` permits mutation, and shadowing uses `let` to create a new binding (possibly with a new type). Use `const` for compile-time constants. Integers, floats, `bool`, and `char` are scalar types; arrays have fixed size, tuples combine differently typed values, and slices borrow part of a collection. `String` owns growable UTF-8 text; `&str` is a string slice. Use `TryFrom` for checked conversions rather than assuming every `as` cast is lossless. A function's final expression is its return value; `;` turns an expression into a statement.

## Explanation and Runnable Example
```rust
fn display_name(given: &str, family: &str) -> String {
    format!("{given} {family}")
}
fn main() {
    let name = display_name("Ada", "Example");
    println!("{name}");
}
```
Create a scratch binary with `cargo new rust-day-01`, then `cargo run`. `cargo check`, `cargo fmt`, `cargo clippy`, and `cargo test` are the everyday feedback loop. `rust-toolchain.toml` selects stable; the package selects edition 2024. The package/crate/module distinction is developed on Day 08.

Type experiment: `let mut retries = 0_u8; retries += 1; let retries = retries.to_string();` demonstrates mutability followed by shadowing. A checked integer conversion can fail; represent that with `Result`, not a silent truncating cast.

## Main Project
Represent an observation as ordinary values before introducing structs. This keeps syntax and expressions in focus.

## Coding Exercises
1. Write `observation_label(code: &str, value: f64, unit: &str) -> String`; print `"HR: 72 bpm"`. Try omitting the return type and observe what the compiler can infer locally.

## Quiz
1. Is a `let` binding mutable by default?
2. What does the semicolon after an expression do?

## Test Your Understanding
Run the program, then deliberately add `;` after `format!` and read the diagnostic.

## Reference Solutions
See instructor exercise solution Day 01; attempt first.

## Common Mistakes
Confusing a binding with the value; adding `mut` without a mutation; assuming `String` and `&str` are the same type.

## Production Perspective
Use formatted strings for presentation, not as the only representation of structured domain data.

## Architecture Perspective
Keep today's program in one function. A boundary is valuable only after there is a reason to separate behavior.

## Project Change
### What We Are Adding
A display-only patient/observation vocabulary.
### Why We Need It
It anchors examples to the application without prematurely designing a domain model.
### Files Added
Scratch Cargo binary; no service files yet.
### Files Changed
None in the reference backend.
### Rust Concepts Used
Bindings, literals, inferred types, functions, formatting.
### Tests Added
Run the binary and inspect output.
### Architecture Impact
None; the system is intentionally a small program.

## Day Summary
Cargo builds a crate; immutable bindings and expression returns are normal Rust.

## What You Should Be Able to Do Now
Write and run a small Rust function that formats synthetic observation data.
