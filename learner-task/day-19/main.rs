use std::time::Duration;

async fn answer_after_a_short_wait() -> u32 {
    // Awaiting a timer lets Tokio work on another ready task instead of blocking this thread.
    tokio::time::sleep(Duration::from_millis(20)).await;
    42
}

#[tokio::main]
async fn main() {
    // Calling an async function creates a future; awaiting it lets the runtime drive it forward.
    let first = answer_after_a_short_wait();
    let second = answer_after_a_short_wait();
    let (first, second) = tokio::join!(first, second);

    println!("Answers: {first} and {second}");
    println!("These tasks share a runtime; async does not mean one new thread per task.");
}
