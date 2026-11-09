# Day 06 — Collections and owned strings

**Time box:** 55 minutes.

## Learning Objectives
- Select between `Vec`, `HashMap`, and `HashSet`.
- Explain how collections own their contents.

## Why This Matters
Patient and observation lists are owned collections, while APIs can borrow their elements.

## Concepts
`Vec<T>` is a growable sequence; `HashMap<K,V>` maps keys to values; `HashSet<T>` tracks unique values. `String` owns UTF-8 bytes; indexing by integer is disallowed because UTF-8 characters are variable length.

## Explanation and Runnable Example
```rust
use std::collections::HashMap;
fn main() {
    let mut counts = HashMap::new();
    for code in ["HR", "TEMP", "HR"] {
        *counts.entry(code).or_insert(0_u32) += 1;
    }
    assert_eq!(counts["HR"], 2);
}
```

## Main Project
Use a vector for ordered observation pages and a map only when keyed lookup is needed.

## Coding Exercises
1. Count observation codes from a slice of `&str` values using a `HashMap`; return the count for a requested code.

## Quiz
1. Why can't Rust index a `String` by arbitrary character position?
2. Which collection expresses uniqueness?

## Test Your Understanding
Compare `Vec::push` with `HashMap::entry`; identify which operation needs mutability.

## Reference Solutions
See instructor exercise solution Day 06.

## Common Mistakes
Using hash maps when order matters; assuming `len()` for `String` is the character count.

## Production Perspective
Bound inputs and collection growth when data comes from a request or queue.

## Architecture Perspective
Choose a collection based on behavior and complexity, not familiarity alone.

## Project Change
### What We Are Adding
An in-memory registry/counting exercise.
### Why We Need It
Collections are the substrate for domain processing before a database exists.
### Files Added
Lesson example.
### Files Changed
None in service.
### Rust Concepts Used
`String`, `Vec`, `HashMap`, `HashSet`.
### Tests Added
Check repeated and absent keys.
### Architecture Impact
Data structures encode ordering and lookup needs.

## Day Summary
Collections own elements and have distinct access/ordering trade-offs.

## What You Should Be Able to Do Now
Choose a collection and explain its semantics for a small domain task.
