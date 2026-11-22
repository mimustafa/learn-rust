struct SafeEvent {
    operation: &'static str,
    request_id: u32,
}

fn main() {
    let event = SafeEvent {
        operation: "list_observations",
        request_id: 17,
    };

    // Structured fields are easier to search than a single formatted message.
    println!(
        "operation={} request_id={}",
        event.operation, event.request_id
    );
    println!("Liveness asks whether the process runs; readiness checks its dependencies.");
    println!("Do not add tokens, request bodies, or patient details to this event.");
}
