fn observation_label(code: &str, value: f64, unit: &str) -> String {
    format!("{code}: {value} {unit}")
}

fn main() {
    let label = observation_label("HR", 72.0, "bpm");
    println!("{label}");
}
