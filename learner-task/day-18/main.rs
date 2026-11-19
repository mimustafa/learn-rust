fn required_setting(value: Option<&str>, name: &str) -> Result<String, String> {
    match value.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => Ok(value.to_owned()),
        None => Err(format!("required setting {name} is missing")),
    }
}

fn main() {
    // The sample values keep this lesson runnable; real secrets should come from configuration.
    let database_url = required_setting(Some("postgres://localhost/example"), "DATABASE_URL");
    let api_token = required_setting(None, "API_TOKEN");
    for (name, setting) in [("DATABASE_URL", database_url), ("API_TOKEN", api_token)] {
        match setting {
            Ok(_) => println!("{name} is configured"),
            Err(message) => println!("Configuration error: {message}"),
        }
    }

    // Knowing who made a request does not, by itself, grant access to every patient's data.
    println!("Authentication identifies a caller; authorization checks what they may do.");
}
