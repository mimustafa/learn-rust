struct CreatePatientRequest {
    full_name: String,
}

fn validate_name(name: &str) -> Result<(), &'static str> {
    if name.trim().is_empty() {
        Err("name cannot be empty")
    } else if name.len() > 80 {
        Err("name cannot exceed 80 bytes")
    } else {
        Ok(())
    }
}

fn handle_create(request: CreatePatientRequest) -> Result<String, &'static str> {
    // A parsed request is still untrusted until the domain rule accepts its contents.
    validate_name(&request.full_name)?;
    Ok(format!("201 Created: {}", request.full_name.trim()))
}

fn main() {
    let requests = [
        CreatePatientRequest {
            full_name: String::from("Ada Example"),
        },
        CreatePatientRequest {
            full_name: String::from(" "),
        },
    ];

    for request in requests {
        println!("{:?}", handle_create(request));
    }
}
