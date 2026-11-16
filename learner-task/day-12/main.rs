fn normalize_name(input: &str) -> Result<String, &'static str> {
    let name = input.trim();
    if name.is_empty() {
        return Err("name cannot be empty");
    }
    if name.len() > 80 {
        return Err("name cannot exceed 80 bytes");
    }
    Ok(name.to_owned())
}

fn main() {
    for input in [" Ada ", "", "Example Patient"] {
        println!("{input:?} -> {:?}", normalize_name(input));
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_name;

    #[test]
    fn trims_surrounding_whitespace() {
        assert_eq!(normalize_name(" Ada "), Ok(String::from("Ada")));
    }

    #[test]
    fn rejects_empty_names() {
        assert_eq!(normalize_name("  "), Err("name cannot be empty"));
    }

    #[test]
    fn rejects_names_over_the_limit() {
        let long_name = "a".repeat(81);
        assert_eq!(
            normalize_name(&long_name),
            Err("name cannot exceed 80 bytes")
        );
    }
}
