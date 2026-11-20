use std::time::Duration;

async fn finish_quickly() -> &'static str {
    tokio::time::sleep(Duration::from_millis(5)).await;
    "finished"
}

async fn take_too_long() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    match tokio::time::timeout(Duration::from_millis(100), finish_quickly()).await {
        Ok(message) => println!("Quick task: {message}"),
        Err(_) => println!("Quick task timed out"),
    }

    match tokio::time::timeout(Duration::from_millis(10), take_too_long()).await {
        Ok(()) => println!("Slow task finished in time"),
        Err(_) => println!("Slow task timed out"),
    }

    println!("A timeout stops waiting locally; it cannot undo a remote side effect.");
}
