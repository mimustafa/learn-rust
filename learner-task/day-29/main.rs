struct ReviewItem {
    risk: &'static str,
    mitigation: &'static str,
    verification: &'static str,
}

fn main() {
    let review = [
        ReviewItem {
            risk: "malformed input",
            mitigation: "parse and validate before using values",
            verification: "test invalid and boundary inputs",
        },
        ReviewItem {
            risk: "missing credential",
            mitigation: "reject protected requests without a credential",
            verification: "test the unauthenticated response",
        },
        ReviewItem {
            risk: "sensitive details in logs",
            mitigation: "log safe operation context only",
            verification: "inspect emitted fields for tokens and request bodies",
        },
    ];

    // Each concern is paired with something an engineer can check, not just a promise.
    for item in review {
        println!(
            "{} -> {} -> verify: {}",
            item.risk, item.mitigation, item.verification
        );
    }
    println!("Passing tests do not make this educational sample production-ready.");
}
