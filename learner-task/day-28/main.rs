struct BoundaryProposal {
    requirement: &'static str,
    owner: &'static str,
    contract: &'static str,
    failure_cost: &'static str,
}

fn main() {
    let proposal = BoundaryProposal {
        requirement: "independent observation-ingestion scaling",
        owner: "not yet established",
        contract: "versioned HTTP API with bounded retries",
        failure_cost: "network failures, duplicate delivery, and extra operations",
    };

    println!("Requirement: {}", proposal.requirement);
    println!("Proposed owner: {}", proposal.owner);
    println!("Contract: {}", proposal.contract);
    println!("New failure costs: {}", proposal.failure_cost);
    println!("Without a clear need and owner, keep this feature in the modular monolith.");
}
