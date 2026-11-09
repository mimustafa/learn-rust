# Day 03 — Ownership, moves, and copying

**Time box:** 60 minutes.

## Learning Objectives
- Explain what it means for a binding to own a value.
- Predict when a move occurs and distinguish `Copy` from explicit `Clone`.

## Why This Matters
Rust makes resource ownership visible at compile time. Most strings and collections move; simple scalar values such as `i32` are `Copy`.

## Concepts
After moving a `String`, the old binding cannot be used. `clone()` allocates/copies when the type implements it; do not use it as an automatic compiler-error eraser. Assignment can transfer ownership.

## Explanation and Runnable Example
```rust
fn consume(name: String) { println!("{name}"); }
fn main() {
    let name = String::from("Ada Example");
    consume(name);
    // println!("{name}"); // Uncomment: the value was moved.
}
```
Compiler-error exercise: uncomment the final line, run `rustc`, then decide whether the function should consume or borrow.

## Main Project
Choose ownership for patient display data versus functions that only read it.

## Coding Exercises
1. Repair the example so `main` can print `name` after calling the display function, without cloning.

## Quiz
1. Does passing an `i32` to a function normally make the caller's binding unusable?
2. What should you decide before adding `.clone()`?

## Test Your Understanding
Create two bindings from a `String` and predict which one owns the buffer after assignment.

## Reference Solutions
See instructor exercise solution Day 03.

## Common Mistakes
Assuming all assignments copy; cloning every moved value; confusing ownership with garbage collection.

## Production Perspective
Avoiding unnecessary copies can help, but make API intent clear before optimizing.

## Architecture Perspective
An API that consumes a value expresses transfer; a read-only API should usually borrow.

## Project Change
### What We Are Adding
An ownership decision for formatted patient data.
### Why We Need It
Ownership shapes every later domain and handler API.
### Files Added
Lesson-only example.
### Files Changed
None in service.
### Rust Concepts Used
Move, `Copy`, `Clone`, ownership transfer.
### Tests Added
Compile a deliberate moved-value failure.
### Architecture Impact
Function signatures express who retains data.

## Day Summary
Each non-`Copy` value has one owner; a move transfers that responsibility.

## What You Should Be Able to Do Now
Recognize an ownership transfer and choose between consuming and borrowing intentionally.
