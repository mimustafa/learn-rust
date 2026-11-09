# Day 04 — Borrowing, references, and slices

**Time box:** 60 minutes.

## Learning Objectives
- Use shared and mutable references safely.
- Explain the aliasing rule and use a string slice.

## Why This Matters
Borrowing lets functions use data without taking ownership or making unnecessary copies.

## Concepts
Many `&T` readers may coexist, or one `&mut T` writer may exist, but not both at once. References must not outlive their data. `&str` can view a `String` or a string literal.

## Explanation and Runnable Example
```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
fn main() {
    let note = String::from("heart rate");
    println!("{}", first_word(&note));
    println!("{note}");
}
```

## Main Project
Reporting code borrows observation records; it does not own or mutate the source collection.

## Coding Exercises
1. Implement `longest_name<'a>(left: &'a str, right: &'a str) -> &'a str` by byte length; keep both inputs usable afterward.

## Quiz
1. Can a live shared reference coexist with a mutable reference to the same value?
2. Why is `&str` useful as a read-only function parameter?

## Test Your Understanding
Try to mutate a `String` while a slice into it is still used. Explain the rejected aliasing.

## Reference Solutions
See instructor exercise solution Day 04.

## Common Mistakes
Returning a reference to a local value; treating a lifetime as a runtime allocation.

## Production Perspective
Borrow input when the function does not need to retain it. Return owned data when output must outlive inputs.

## Architecture Perspective
Borrowing is an API choice, not merely a compiler workaround.

## Project Change
### What We Are Adding
Borrowed reporting functions over owned records.
### Why We Need It
Read paths should not consume domain values.
### Files Added
Lesson-only example.
### Files Changed
None in service.
### Rust Concepts Used
`&T`, `&mut T`, `&str`, slices, aliasing.
### Tests Added
Compiler experiment for mutation while borrowed.
### Architecture Impact
Separates observation from ownership.

## Day Summary
Borrow to access; choose mutability and lifetime relationships deliberately.

## What You Should Be Able to Do Now
Design a read-only function that accepts references and returns data with a valid lifetime.
