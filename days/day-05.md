# Day 05 — Structs, enums, `Option`, and `Result`

**Time box:** 60 minutes.

## Learning Objectives
- Model a patient with a struct and a finite state with an enum.
- Represent missing data with `Option` and failure with `Result`.

## Why This Matters
Rust types make assumptions explicit. Missing birth date is not an error; invalid input is.

## Concepts
Struct fields are named; enums are sum types. `Option<T>` is `Some(T)` or `None`; `Result<T,E>` is `Ok(T)` or `Err(E)`. Pattern matching handles each case.

## Explanation and Runnable Example
```rust
struct Patient { name: String, birth_year: Option<u16> }
fn display_year(year: Option<u16>) -> String {
    match year { Some(value) => value.to_string(), None => "unknown".into() }
}
fn main() {
    let patient = Patient { name: "Ada".into(), birth_year: None };
    println!("{}: {}", patient.name, display_year(patient.birth_year));
}
```

## Main Project
Define patient and observation as distinct values. The final reference adds validated input and timestamps.

## Coding Exercises
1. Add an enum `EncounterState` with `Scheduled`, `InProgress`, and `Completed`; write an exhaustive label function.
2. Write a function that returns `Result<u32, &'static str>` for parsing a positive count.

## Quiz
1. Which type means "may be absent"?
2. Which type means "operation may fail with an explanation"?

## Test Your Understanding
What new compiler work is required if another `EncounterState` variant is added?

## Reference Solutions
See instructor exercise solution Day 05.

## Common Mistakes
Using sentinel strings or zero for missing values; using `Result` to represent ordinary absence.

## Production Perspective
Public input needs validation even when represented by a Rust type.

## Architecture Perspective
Choose types that reveal domain meaning rather than primitive values everywhere.

## Project Change
### What We Are Adding
Patient/observation shapes and explicit optional/state values.
### Why We Need It
The service needs a vocabulary before storage and HTTP.
### Files Added
Lesson example; see `project/final/src/domain.rs`.
### Files Changed
Reference domain module.
### Rust Concepts Used
Structs, enums, `Option`, `Result`, `match`.
### Tests Added
Parsing and state-label exercises.
### Architecture Impact
Domain values replace ad hoc parameter lists.

## Day Summary
Types express what may be absent, what may vary, and what may fail.

## What You Should Be Able to Do Now
Model a small domain record without null sentinels or arbitrary state strings.
