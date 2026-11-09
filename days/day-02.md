# Day 02 — Expressions, branches, and exhaustive matching

**Time box:** 55 minutes.

## Learning Objectives
- Distinguish expressions from statements and use `if` as an expression.
- Use `match` to handle every variant of a domain state.

## Why This Matters
Domain decisions should be visible in code; exhaustive matching makes newly introduced states hard to overlook.

## Concepts
`if` branches return compatible types. `match` must cover all possibilities. `_` is a deliberate catch-all; avoid it when each business state needs different behavior. Rust also has `if let`/`let else` for a focused pattern, `loop` with `break`/`continue`, condition-driven `while`, `while let` for repeated pattern extraction, and `for` iteration over ranges or iterators.

## Explanation
```rust
#[derive(Debug)]
enum Review { Pending, Accepted, Rejected }
fn label(status: Review) -> &'static str {
    match status {
        Review::Pending => "pending",
        Review::Accepted => "accepted",
        Review::Rejected => "rejected",
    }
}
fn main() { println!("{}", label(Review::Pending)); }
```
The program is complete and runs with `rustc day02.rs`; enums are introduced today only to demonstrate the compiler checking alternatives. `if let Some(value) = maybe_value { ... }` avoids a full `match` when only one case matters. Prefer `for item in items` over manual index loops when iteration is the goal.

## Main Project
Use a controlled review state for an incoming observation; never represent it as an arbitrary string.

## Coding Exercises
1. Add `Archived` to `Review`; make `label` compile and return `"archived"`.
2. Write a function classifying a finite `f64` as `"negative"` or `"non-negative"` and non-finite values as `"invalid"`.

## Quiz
1. Why does adding an enum variant produce a useful compile error?
2. When is `_` a poor match arm?

## Test Your Understanding
Remove one match arm, compile, and explain the compiler's diagnostic.

## Reference Solutions
See instructor exercise solution Day 02.

## Common Mistakes
Treating `match` as a switch with optional defaults; ignoring NaN when comparing floats.

## Production Perspective
Reject invalid/non-finite measurement values at the boundary, before persistence.

## Architecture Perspective
An enum makes the set of possible states explicit and reviewable.

## Project Change
### What We Are Adding
An example status enum and finite-number check.
### Why We Need It
Controlled states and measurement validation recur in the service.
### Files Added
Lesson-only example.
### Files Changed
No reference backend change.
### Rust Concepts Used
Enums, exhaustive `match`, conditions, float edge cases.
### Tests Added
Compiler coverage exercise; extend with assertions.
### Architecture Impact
Domain states are modeled rather than free-form strings.

## Day Summary
Exhaustiveness is an executable design check.

## What You Should Be Able to Do Now
Add a domain state and use compiler feedback to update every match.
