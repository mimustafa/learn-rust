# Day 10 — Lifetimes and reference relationships

**Time box:** 60 minutes.

## Learning Objectives
- Read lifetime annotations as relationships between references.
- Avoid unnecessary annotations by understanding elision.

## Why This Matters
Lifetimes prevent dangling references; they do not extend a value's storage duration.

## Concepts
The compiler infers many function lifetimes. Add a named lifetime when the return reference is tied to an input. A struct containing references must state the relationship; owning a `String` often makes an API simpler.

Lifetime elision covers common function signatures such as `fn first(text: &str) -> &str`. Returning a reference is safe only when its source outlives the returned borrow. A struct holding borrowed text needs a lifetime parameter (`struct Label<'a> { text: &'a str }`); compare that with an owned `String` before choosing.

## Explanation and Runnable Example
```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
fn main() { println!("{}", longer("heart", "rate")); }
```
The signature promises the returned slice is valid for the shared input lifetime, not that either input lives forever.

## Main Project
The reference service owns database response strings, so response values can outlive a query borrow.

## Coding Exercises
1. Write `first_word(text: &str) -> &str` without explicit lifetime syntax. Explain why elision works.

## Quiz
1. Does `'a` allocate data?
2. Why can't a function return a reference to its local `String`?

## Test Your Understanding
Try returning one of two inputs with different lifetimes; ask what contract the caller can rely on.

## Reference Solutions
See instructor exercise solution Day 10.

## Common Mistakes
Treating annotations as runtime checks; trying to store short-lived request references in long-lived application state.

## Production Perspective
Owned DTOs at async/network boundaries can simplify validity and task transfer.

## Architecture Perspective
Choose borrowing for local APIs when the relationship is clear; choose ownership when it makes the boundary simpler.

## Project Change
### What We Are Adding
Explicit lifetime reasoning for report helpers.
### Why We Need It
It makes ownership decisions legible before async APIs grow.
### Files Added
Lesson-only examples.
### Files Changed
No explicit lifetime annotations required by final service.
### Rust Concepts Used
Lifetime elision, named lifetime parameters, borrowed output.
### Tests Added
Compile the returned-slice example.
### Architecture Impact
The service prefers owned response DTOs across the I/O boundary.

## Day Summary
Lifetimes describe validity relationships; ownership determines storage responsibility.

## What You Should Be Able to Do Now
Read a simple lifetime signature without believing it changes runtime behavior.
