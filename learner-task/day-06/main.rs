use std::collections::{HashMap, HashSet};

fn count_code(codes: &[&str], requested: &str) -> usize {
    let mut counts = HashMap::new();
    for code in codes {
        *counts.entry(*code).or_insert(0) += 1;
    }
    counts.get(requested).copied().unwrap_or(0)
}

fn main() {
    let codes = ["HR", "TEMP", "HR", "BP"];

    // A Vec preserves the order in which the observations arrived.
    let observations = codes.to_vec();
    println!("Observation order: {observations:?}");

    // A HashSet is useful when only distinct codes matter.
    let unique_codes: HashSet<_> = observations.iter().copied().collect();
    println!("Unique codes: {unique_codes:?}");
    println!("HR appears {} times", count_code(&codes, "HR"));
    println!("O2 appears {} times", count_code(&codes, "O2"));

    // String length counts UTF-8 bytes, not human-visible characters.
    let greeting = String::from("café");
    println!("'café' uses {} UTF-8 bytes", greeting.len());
}
