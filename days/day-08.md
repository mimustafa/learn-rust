# Day 08 — Cargo packages, modules, and visibility

**Time box:** 60 minutes.

## Learning Objectives
- Distinguish package, crate, and module.
- Use `mod`, `use`, and `pub` to establish a small boundary.

## Why This Matters
As the service grows, file and visibility boundaries keep HTTP and domain rules understandable.

## Concepts
A package is managed by Cargo and may contain several crates; a crate is a compilation unit; modules organize names in a crate. Items are private by default. `pub` exposes an item through its module path.

## Explanation
```rust
mod patient {
    pub struct Patient { pub id: u64 }
    pub fn new(id: u64) -> Patient { Patient { id } }
}
fn main() {
    let patient = patient::new(1);
    println!("{}", patient.id);
}
```
Keep the module small. `pub(crate)` is useful when an item is needed across internal modules but should not be public API.

## Main Project
Inspect `project/final/src`; identify which module knows about HTTP and which validates names.

## Coding Exercises
1. Split a scratch patient example into `patient` and `main` modules; expose only construction and the field required by the caller.

## Quiz
1. What is the default visibility of a module item?
2. Can one package contain a binary and library crate?

## Test Your Understanding
Make a field private and use the compiler's suggested boundary feedback.

## Reference Solutions
See instructor exercise solution Day 08.

## Common Mistakes
Making everything public; using modules as folders without controlling APIs.

## Production Perspective
Public Rust APIs are commitments; minimize visibility to reduce accidental coupling.

## Architecture Perspective
Module boundaries should follow domain responsibilities, not arbitrary line counts.

## Project Change
### What We Are Adding
Domain/configuration/routes modules.
### Why We Need It
Each concern becomes independently navigable as the application grows.
### Files Added
`project/final/src/*.rs`.
### Files Changed
Reference package organization.
### Rust Concepts Used
Crates, modules, paths, visibility.
### Tests Added
Compile visibility exercise.
### Architecture Impact
Sets dependency direction for later persistence and HTTP.

## Day Summary
Rust privacy is a design tool; expose the smallest usable interface.

## What You Should Be Able to Do Now
Organize a small Rust package into modules with intentional public items.
