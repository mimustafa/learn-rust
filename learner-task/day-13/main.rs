fn validate_name(input: &str) -> Result<&str, &'static str> {
    let name = input.trim();
    if name.is_empty() {
        return Err("name cannot be empty");
    }
    if name.len() > 80 {
        return Err("name cannot exceed 80 bytes");
    }
    Ok(name)
}

fn main() {
    // Domain validation stays independent of HTTP and database types.
    for name in [" Ada Example ", "  "] {
        match validate_name(name) {
            Ok(valid_name) => println!("Accepted name: {valid_name}"),
            Err(message) => println!("Rejected name: {message}"),
        }
    }
}
