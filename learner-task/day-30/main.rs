use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreatePatientRequest {
    full_name: String,
}

fn main() {
    let valid_json = r#"{"full_name":"Ada Example"}"#;
    let request: CreatePatientRequest =
        serde_json::from_str(valid_json).expect("known request fields should parse");
    println!("Accepted patient name: {}", request.full_name);

    // Unknown fields are rejected before this example could reach a database.
    let unknown_field = r#"{"full_name":"Ada Example","admin":true}"#;
    match serde_json::from_str::<CreatePatientRequest>(unknown_field) {
        Ok(_) => println!("Unexpectedly accepted an unknown field"),
        Err(error) => println!("Rejected unknown field: {error}"),
    }

    println!("The request shape is explicit; it still needs domain validation.");
}

#[cfg(test)]
mod tests {
    use super::CreatePatientRequest;

    #[test]
    fn rejects_unknown_request_fields() {
        let json = r#"{"full_name":"Ada Example","admin":true}"#;
        assert!(serde_json::from_str::<CreatePatientRequest>(json).is_err());
    }
}
