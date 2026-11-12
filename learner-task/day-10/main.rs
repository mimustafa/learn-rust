// Rust can infer this lifetime relationship for a simple input-to-output borrow.
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// The named lifetime makes clear that the answer comes from one of these inputs.
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

struct Label<'a> {
    text: &'a str,
}

fn main() {
    let note = String::from("heart rate");
    println!("First word: {}", first_word(&note));
    println!("Longer text: {}", longer("heart", "rate"));

    // This label borrows note, so note must stay alive while the label is used.
    let label = Label { text: &note };
    println!("Borrowed label: {}", label.text);
}
