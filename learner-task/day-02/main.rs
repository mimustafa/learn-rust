#[derive(Debug)]
enum Review {
    Pending,
    Accepted,
    Rejected,
    Archived,
}

// Matching every state means Rust points out any new state we forget to handle.
fn review_label(status: Review) -> &'static str {
    match status {
        Review::Pending => "pending",
        Review::Accepted => "accepted",
        Review::Rejected => "rejected",
        Review::Archived => "archived",
    }
}

// NaN and infinity are not meaningful measurements, so reject them before comparing.
fn classify_value(value: f64) -> &'static str {
    if !value.is_finite() {
        "invalid"
    } else if value < 0.0 {
        "negative"
    } else {
        "non-negative"
    }
}

fn main() {
    // Each variant is handled by review_label, including the newly added Archived state.
    let statuses = [
        Review::Pending,
        Review::Accepted,
        Review::Rejected,
        Review::Archived,
    ];
    for status in statuses {
        println!("Review status: {}", review_label(status));
    }

    // These examples cover a negative number, zero, and a non-finite value.
    println!("  -2.5: {}", classify_value(-2.5));
    println!("   0.0: {}", classify_value(0.0));
    println!("   NaN: {}", classify_value(f64::NAN));
}
