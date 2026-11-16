fn parse_positive_count(input: &str) -> Result<u32, &'static str> {
    let count = input
        .parse::<u32>()
        .map_err(|_| "count must be a whole number")?;

    if count == 0 {
        return Err("count must be greater than zero");
    }

    Ok(count)
}

fn main() {
    for input in ["4", "0", "many"] {
        // Result forces the caller to choose what to do for both success and failure.
        match parse_positive_count(input) {
            Ok(count) => println!("{input:?} became {count}"),
            Err(message) => println!("{input:?} was rejected: {message}"),
        }
    }
}
