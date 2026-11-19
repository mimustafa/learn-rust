struct PatientQuery {
    id: u64,
}

fn find_patient_query(id: u64) -> (&'static str, PatientQuery) {
    // Keep SQL text fixed and pass user-controlled values separately as parameters.
    (
        "SELECT id, full_name FROM patients WHERE id = $1",
        PatientQuery { id },
    )
}

fn main() {
    let (sql, parameters) = find_patient_query(42);
    println!("SQL: {sql}");
    println!("Bound patient ID: {}", parameters.id);
    println!("A real SQLx call would bind this ID and await the database response.");
}
