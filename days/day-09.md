# Day 09 — Generics, traits, and deliberate abstraction

**Time box:** 60 minutes.

## Learning Objectives
- Define a generic function with a trait bound.
- Compare static dispatch with `dyn Trait` and identify when a concrete type is enough.

## Why This Matters
Traits enable shared behavior and test seams, but extra abstraction has a cost.

## Concepts
`T: Trait` is generally monomorphized (static dispatch). `&dyn Trait` uses dynamic dispatch and needs object-safe behavior (no generic method that would require an unknown vtable entry). `impl Trait` in an argument is shorthand for a bound; in a return position it hides one concrete type. Associated types name a trait's chosen related type, and blanket implementations apply behavior to every type meeting a bound. The newtype pattern gives a primitive domain meaning; type-state can encode a small compile-time workflow when it materially prevents misuse. `PhantomData` is an advanced marker, not a default modeling tool. Add abstraction only when generic reuse, runtime heterogeneity, or a real test/substitution seam benefits the design.

## Explanation and Runnable Example
```rust
trait HasCode { fn code(&self) -> &str; }
struct Observation { code: String }
impl HasCode for Observation { fn code(&self) -> &str { &self.code } }
fn code_of<T: HasCode>(item: &T) -> &str { item.code() }
fn main() {
    let item = Observation { code: "HR".into() };
    println!("{}", code_of(&item));
}
```

## Main Project
Keep SQLx calls concrete until an actual substitution/testing need appears. Do not define a trait per table.

## Coding Exercises
1. Add a second type implementing `HasCode`; write a generic function that returns its code.

## Quiz
1. What does a trait bound guarantee to generic code?
2. Why may a concrete struct be simpler than a trait object?

## Test Your Understanding
Replace `T: HasCode` with `&dyn HasCode`; note the caller/runtime trade-off.

## Reference Solutions
See instructor exercise solution Day 09.

## Common Mistakes
Traits as C# interfaces by reflex; using `dyn` without needing runtime heterogeneity.

## Production Perspective
Use a trait where consumers need a stable behavior boundary, not just because mocking is familiar.

## Architecture Perspective
Abstractions trade flexibility for complexity and indirection.

## Project Change
### What We Are Adding
A behavior interface example and a decision not to over-abstract persistence.
### Why We Need It
Type-driven design includes knowing when not to add a trait.
### Files Added
Lesson example.
### Files Changed
No trait layer in final app.
### Rust Concepts Used
Generics, bounds, static/dynamic dispatch.
### Tests Added
Compile two implementations through one generic function.
### Architecture Impact
Keeps the service concrete until substitution is justified.

## Day Summary
Traits describe capabilities; generics and trait objects make different dispatch trade-offs.

## What You Should Be Able to Do Now
Defend a trait boundary—or explain why none is needed.
