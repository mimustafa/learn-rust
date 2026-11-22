use std::{hint::black_box, time::Instant};

fn main() {
    let original = String::from("HR: 72 bpm");
    let iterations = 100_000;
    let start = Instant::now();

    for _ in 0..iterations {
        // black_box keeps the result observable so the optimizer cannot discard the work.
        black_box(original.clone());
    }

    let elapsed = start.elapsed();
    println!("Cloned a short label {iterations} times in {elapsed:?}");
    println!("This quick timing is only a demonstration, not a representative service benchmark.");
}
