struct Patient {
    name: String,
    birth_year: Option<u16>,
}

#[derive(Clone, Copy)]
enum EncounterState {
    Scheduled,
    InProgress,
    Completed,
}

fn state_label(state: EncounterState) -> &'static str {
    match state {
        EncounterState::Scheduled => "scheduled",
        EncounterState::InProgress => "in progress",
        EncounterState::Completed => "completed",
    }
}

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
    let patient = Patient {
        name: String::from("Ada Example"),
        birth_year: None,
    };

    // Option makes missing data visible instead of hiding it in a special string or number.
    let year = patient
        .birth_year
        .map_or_else(|| "unknown".to_string(), |year| year.to_string());
    println!("{}; birth year: {year}", patient.name);

    for state in [
        EncounterState::Scheduled,
        EncounterState::InProgress,
        EncounterState::Completed,
    ] {
        println!("Encounter: {}", state_label(state));
    }

    match parse_positive_count("3") {
        Ok(count) => println!("Parsed count: {count}"),
        Err(message) => println!("Invalid count: {message}"),
    }
}
