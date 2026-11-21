# Day 21 — Threads, channels, and parallelism

**Time box:** 55 minutes.

## Learning Objectives
- Spawn a standard thread and communicate through a channel.
- Distinguish CPU parallelism from async I/O concurrency.

## Why This Matters
CPU-heavy work may need parallel execution; I/O waits often fit async tasks better.

## Concepts
`std::thread::spawn` creates an OS thread. Standard channels transfer owned messages. `join` observes worker completion. A bounded worker pool is safer than spawning unbounded threads for user requests.

## Explanation and Runnable Example
```rust
use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(6 * 7).expect("receiver is alive"));
    println!("{}", rx.recv().expect("worker sent a value"));
}
```

## Main Project
Do not replace async HTTP I/O with one thread per request. Identify where actual CPU parallelism could be justified.

## Coding Exercises
1. Send three owned numeric observations through a channel to one worker and compute their sum.

## Quiz
1. Which matters more for CPU parallelism: task or OS thread?
2. What is a benefit of message passing over shared mutation?

## Test Your Understanding
What happens if the receiver is dropped before send?

## Reference Solutions
See instructor exercise solution Day 21.

## Common Mistakes
Creating unbounded threads; ignoring channel closure; confusing concurrency with parallelism.

## Production Perspective
Bound worker count and queue size; monitor queue delay and shutdown behavior.

## Architecture Perspective
Prefer one clear owner and messages when that removes synchronization complexity.

## Project Change
### What We Are Adding
A standalone thread/channel experiment.
### Why We Need It
To compare threads with Tokio tasks before selecting tools.
### Files Added
Lesson example only.
### Files Changed
No extra thread pool in the HTTP reference.
### Rust Concepts Used
Threads, move closures, channels, joining.
### Tests Added
Check message result and receiver closure.
### Architecture Impact
No production thread pool without a workload and capacity bound.

## Day Summary
Use async for many I/O waits; use bounded threads for CPU work where measurement supports it.

## What You Should Be Able to Do Now
Send owned work to a thread and explain its resource cost.
