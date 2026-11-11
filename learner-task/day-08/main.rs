mod patient {
    // Only the ID is exposed; callers cannot directly rewrite the stored field.
    pub struct Patient {
        pub id: u64,
        name: String,
    }

    pub fn new(id: u64, name: String) -> Patient {
        Patient { id, name }
    }

    pub fn display_name(patient: &Patient) -> &str {
        &patient.name
    }
}

fn main() {
    let patient = patient::new(1, String::from("Ada Example"));
    println!(
        "Patient {}: {}",
        patient.id,
        patient::display_name(&patient)
    );
}
