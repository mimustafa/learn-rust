// Borrowing the text lets this function format it without taking ownership.
fn observation_label(code: &str, value: f64, unit: &str) -> String {
    format!("{code}: {value} {unit}")
}

fn main() {
    // The returned String owns the finished label, so it can be printed later.
    let label = observation_label("HR", 72.0, "bpm");
    println!("{label}");
}
