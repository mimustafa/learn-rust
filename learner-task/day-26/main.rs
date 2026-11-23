struct FailureCase {
    situation: &'static str,
    response: &'static str,
    safe_signal: &'static str,
}

fn main() {
    let cases = [
        FailureCase {
            situation: "invalid request name",
            response: "400 Bad Request",
            safe_signal: "validation failure count",
        },
        FailureCase {
            situation: "database unavailable",
            response: "503 Service Unavailable",
            safe_signal: "database health and error count",
        },
        FailureCase {
            situation: "page limit too large",
            response: "400 Bad Request",
            safe_signal: "rejected page-limit count",
        },
    ];

    // A failure table makes the response and operator signal explicit for each case.
    for case in cases {
        println!(
            "{} -> {}; record {}",
            case.situation, case.response, case.safe_signal
        );
    }
    println!("A connection-pool cap does not replace request limits or a database timeout.");
}
