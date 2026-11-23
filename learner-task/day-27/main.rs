mod domain {
    // Domain rules depend only on Rust's standard types, not on HTTP or SQL.
    pub fn validate_name(name: &str) -> Result<&str, &'static str> {
        let name = name.trim();
        if name.is_empty() {
            Err("name cannot be empty")
        } else {
            Ok(name)
        }
    }
}

mod application {
    use super::domain;

    pub fn create_patient(name: &str) -> Result<String, &'static str> {
        let name = domain::validate_name(name)?;
        Ok(format!("Prepared patient: {name}"))
    }
}

fn main() {
    println!("{:?}", application::create_patient(" Ada Example "));
    println!("{:?}", application::create_patient(" "));
    println!(
        "Add a new module or trait only when it gives a real responsibility or substitution seam."
    );
}
