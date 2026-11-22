use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Capacity two means the sender must wait if the consumer falls behind.
    let (sender, mut receiver) = mpsc::channel(2);
    let producer = tokio::spawn(async move {
        for value in [60, 72, 80] {
            sender.send(value).await.expect("receiver is still open");
            println!("Queued observation: {value}");
        }
    });

    while let Some(value) = receiver.recv().await {
        println!("Worker received: {value}");
    }

    producer.await.expect("producer task did not panic");
    println!("This in-memory queue is bounded, but it does not survive a restart.");
}
