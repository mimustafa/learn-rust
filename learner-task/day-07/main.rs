fn sum_even(values: &[i32]) -> i32 {
    values.iter().filter(|value| **value % 2 == 0).sum()
}

fn main() {
    let readings = vec![60, 71, 72, 81];

    // iter() lends each value to the pipeline, so readings remains available afterward.
    let labels: Vec<String> = readings
        .iter()
        .map(|value| format!("{value} bpm"))
        .collect();
    println!("Labels: {labels:?}");
    println!("Even readings total: {}", sum_even(&readings));

    // iter_mut() is for changing items without consuming the vector.
    let mut adjusted = readings.clone();
    for value in &mut adjusted {
        *value += 1;
    }
    println!("Adjusted readings: {adjusted:?}");
}
