// The returned slice points into the caller's text; it does not allocate a new String.
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// The lifetime says the answer borrows from one of these inputs and cannot outlive them.
fn longest_name<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

// A mutable reference gives this function temporary, exclusive permission to change text.
fn append_unit(value: &mut String, unit: &str) {
    value.push(' ');
    value.push_str(unit);
}

fn main() {
    let note = String::from("heart rate");
    let word = first_word(&note);
    println!("First word: {word}");
    println!("Original note is still available: {note}");

    let given = String::from("Ada");
    let family = String::from("Example");
    println!("Longest name: {}", longest_name(&given, &family));

    // This one reference is mutable, so the function can append the unit in place.
    let mut reading = String::from("72");
    append_unit(&mut reading, "bpm");
    println!("Updated reading: {reading}");
}
