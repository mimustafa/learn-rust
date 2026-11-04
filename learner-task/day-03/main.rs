// A shared reference lets this function read the name without taking it from the caller.
fn display_name(name: &str) {
    println!("Display name: {name}");
}

// This function takes ownership, so the caller gives up the String when calling it.
fn consume_label(label: String) {
    println!("Consumed label: {label}");
}

// i32 is Copy, so passing it does not make the caller's value unavailable.
fn print_score(score: i32) {
    println!("Score: {score}");
}

fn main() {
    let name = String::from("Ada Example");
    display_name(&name);
    println!("Name is still available: {name}");

    let label = String::from("HR: 72 bpm");
    consume_label(label);
    // `label` moved into consume_label, so this program cannot use it again here.

    let score = 72;
    print_score(score);
    println!("Copied score is still available: {score}");
}
