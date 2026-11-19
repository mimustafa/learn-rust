struct Page {
    limit: u32,
    offset: u32,
}

fn page(limit: Option<i64>, offset: Option<i64>) -> Result<Page, &'static str> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    if !(1..=100).contains(&limit) {
        return Err("limit must be between 1 and 100");
    }
    if offset < 0 {
        return Err("offset cannot be negative");
    }

    let limit = u32::try_from(limit).map_err(|_| "limit is out of range")?;
    let offset = u32::try_from(offset).map_err(|_| "offset is out of range")?;
    Ok(Page { limit, offset })
}

fn main() {
    for request in [
        (None, None),
        (Some(100), Some(20)),
        (Some(101), None),
        (None, Some(-1)),
    ] {
        match page(request.0, request.1) {
            Ok(page) => println!(
                "Accepted page with limit {} and offset {}",
                page.limit, page.offset
            ),
            Err(message) => println!("Bad request: {message}"),
        }
    }

    println!("Use a stable sort key so repeated pages have predictable ordering.");
}
