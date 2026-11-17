const MIGRATION: &str = r#"
CREATE TABLE patients (
    id BIGINT PRIMARY KEY,
    full_name TEXT NOT NULL CHECK (length(full_name) BETWEEN 1 AND 80)
);

CREATE TABLE observations (
    id BIGINT PRIMARY KEY,
    patient_id BIGINT NOT NULL REFERENCES patients(id) ON DELETE CASCADE,
    recorded_at TEXT NOT NULL,
    code TEXT NOT NULL,
    value DOUBLE PRECISION NOT NULL
);

CREATE INDEX observations_by_patient_and_time
    ON observations (patient_id, recorded_at DESC);
"#;

fn main() {
    // This prints a migration for inspection; it does not connect to a database.
    println!("{MIGRATION}");
}
